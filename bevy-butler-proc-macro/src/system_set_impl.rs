use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Error, Ident, Item, Stmt,
};

use crate::{
    system_impl::{register_system_set_block, SystemArgs, SystemAttr},
    utils::Parenthesized,
};

pub(crate) struct SystemSetInput {
    pub args: SystemArgs,
    pub stmts: Vec<Stmt>,
}

impl Parse for SystemSetInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let args: Parenthesized<SystemArgs> = input.parse()?;
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

pub(crate) fn macro_impl(mut input: SystemSetInput) -> Result<TokenStream, TokenStream> {
    let mut systems: Vec<(Ident, Option<SystemArgs>)> = Vec::new();

    input.args.require_plugin()?;
    input.args.require_schedule()?;

    for stmt in input.stmts.iter_mut() {
        match stmt {
            Stmt::Item(Item::Fn(item)) => {
                if let Some(sys_attr) = item
                    .attrs
                    .iter()
                    .position(|attr| attr.path().get_ident().is_some_and(|i| i == "system"))
                    .map(|i| item.attrs.remove(i))
                {
                    // Take and parse the system args ourselves
                    let sys_attr =
                        SystemAttr::try_from(&sys_attr).map_err(|e| e.into_compile_error())?;

                    // Make sure it doesn't try to define a plugin/schedule
                    if let Some(args) = &sys_attr.args {
                        if args.plugin.is_some() {
                            return Err(Error::new(args.span, "`#[system]`s can not define a `plugin` that is different from the enclosing `system_set!`").into_compile_error());
                        } else if args.plugin.is_some() {
                            return Err(Error::new(args.span, "`#[system]`s can not define a `schedule` that is different from the enclosing `system_set!`").into_compile_error());
                        }
                    }

                    systems.push((item.sig.ident.clone(), sys_attr.args));
                }
            }
            _ => (),
        }
    }

    let stmts = input.stmts;

    let register_block = register_system_set_block(&systems, &input.args);

    Ok(quote! {
        #(#stmts)*

        #register_block
    })
}
