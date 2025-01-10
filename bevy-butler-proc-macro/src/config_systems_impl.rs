use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token::Paren,
    Error, Expr, Item, MacroDelimiter, Meta, MetaList, Stmt,
};

use crate::{system_impl::SystemArgs, utils::Parenthesized};

#[derive(Debug)]
pub(crate) struct ConfigSystems {
    pub args: SystemArgs,
    pub stmts: Vec<Stmt>,
}

impl Parse for ConfigSystems {
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

impl ToTokens for ConfigSystems {
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
pub(crate) fn config_impl(input: &mut ConfigSystems) -> Result<(), TokenStream> {
    for stmt in input.stmts.iter_mut() {
        match stmt {
            Stmt::Item(Item::Fn(item)) => {
                if let Some(attr) = item.attrs.iter_mut().find(|attr| {
                    attr.path()
                        .get_ident()
                        .map(|i| i.to_string())
                        .is_some_and(|i| i == "system")
                }) {
                    // Found #[system], modify
                    match &mut attr.meta {
                        Meta::List(list) => {
                            // Splat old arguments with new arguments and rewrite attribute
                            let new_args: SystemArgs = syn::parse2(list.tokens.clone())
                                .map_err(|e| Error::new(list.span(), e).into_compile_error())?;

                            list.tokens = input.args.splat(&new_args).to_token_stream();
                        }
                        Meta::Path(path) => {
                            // Replace #[system] with #[system(...)]
                            attr.meta = Meta::List(MetaList {
                                path: path.clone(),
                                delimiter: MacroDelimiter::Paren(Paren(path.span())),
                                tokens: input.args.to_token_stream(),
                            })
                        }
                        meta @ Meta::NameValue(_) => {
                            return Err(Error::new(
                                meta.span(),
                                "Unexpected name-value meta format",
                            )
                            .into_compile_error())
                        }
                    }
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

                    let config = ConfigSystems {
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
                let mut config: ConfigSystems =
                    mc.mac.parse_body().map_err(|e| e.to_compile_error())?;
                config.args = input.args.splat(&config.args);
                mc.mac.tokens = config.to_token_stream();
            }
            _ => (),
        }
    }

    Ok(())
}
