use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    Expr, Item, Stmt,
};

use crate::{
    system_impl::{SystemArgs, SystemAttr},
    utils::Parenthesized,
};

pub(crate) struct ConfigSystemsInput {
    pub args: SystemArgs,
    pub stmts: Vec<Stmt>,
}

impl Parse for ConfigSystemsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args = Parenthesized::parse(input)?;
        let mut stmts = Vec::new();

        while !input.is_empty() {
            stmts.push(input.parse()?);
        }

        Ok(Self {
            args: args.0,
            stmts,
        })
    }
}

impl ToTokens for ConfigSystemsInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let args = &self.args;
        let statements = &self.stmts;
        tokens.append_all(quote! {
            ( #args )

            #( #statements )*
        });
    }
}

/// Implementation for config_systems! and #[config_systems_block]
pub(crate) fn config_impl(input: &mut ConfigSystemsInput) -> Result<(), TokenStream> {
    for stmt in input.stmts.iter_mut() {
        match stmt {
            Stmt::Item(Item::Fn(item)) => {
                if let Some((attr, mut sys_attr)) = item.attrs.iter_mut().find_map(|attr| {
                    if let Ok(sys_attr) = SystemAttr::try_from(&*attr) {
                        return Some((attr, sys_attr));
                    }
                    None
                }) {
                    sys_attr.args = match sys_attr.args {
                        None => Some(input.args.clone()),
                        Some(new_args) => Some(input.args.splat(&new_args)),
                    };

                    *attr = (&sys_attr).into();
                }
            }
            Stmt::Expr(Expr::Block(block), _) => {
                if let Some(attr) = block.attrs.iter_mut().find(|attr| {
                    attr.path()
                        .get_ident()
                        .map(|i| i.to_string())
                        .is_some_and(|i| i == "config_systems_block")
                }) {
                    // Found #[config_systems_block], modify and rewrite
                    let new_args: SystemArgs =
                        attr.parse_args().map_err(|e| e.to_compile_error())?;

                    let config = ConfigSystemsInput {
                        args: input.args.splat(&new_args),
                        stmts: block.block.stmts.clone(),
                    };

                    let new_stmt: Stmt = syn::parse2(quote! {
                        config_systems! { #config }
                    })
                    .map_err(|e| e.to_compile_error())?;

                    *stmt = new_stmt;
                }
            }
            Stmt::Macro(mc) => {
                let mut config: ConfigSystemsInput =
                    mc.mac.parse_body().map_err(|e| e.to_compile_error())?;
                config.args = input.args.splat(&config.args);
                mc.mac.tokens = config.to_token_stream();
            }
            _ => (),
        }
    }

    Ok(())
}
