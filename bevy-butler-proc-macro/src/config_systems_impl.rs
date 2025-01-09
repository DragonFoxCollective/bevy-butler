use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{spanned::Spanned, token::Paren, Error, ExprBlock, Item, MacroDelimiter, Meta, MetaList, Stmt};

use crate::system_impl::SystemArgs;

/// Implementation for #[config_systems] on block expressions.
/// 
/// ```
/// # #![feature(stmt_expr_attributes)]
/// # #![feature(proc_macro_hygiene)]
/// # use bevy_butler_proc_macro::*;
/// # use bevy::prelude::*;
/// #
/// # #[butler_plugin]
/// # pub struct MyPlugin;
/// #
/// #[config_systems(plugin = MyPlugin, schedule = Update)]
/// {
///     #[system(schedule = Startup)]
///     fn on_startup() {
///         info!("Hello, world!");
///     }
/// 
///     #[system]
///     fn on_update(time: Res<Time>) {
///         info!("The current time is {}", time.elapsed_secs());
///     }
/// }
/// ```
pub(crate) fn block_impl(args: &SystemArgs, item: &mut ExprBlock) -> Result<(), TokenStream>
{
    for stmt in item.block.stmts.iter_mut() {
        match stmt {
            Stmt::Item(Item::Fn(item)) => {
                if let Some(attr) = item.attrs.iter_mut().find(|attr| attr.path().get_ident().map(|i| i.to_string()).is_some_and(|i| i == "system") ) {
                    // Found #[system], modify
                    match &mut attr.meta {
                        Meta::List(list) => {
                            // Splat old arguments with new arguments and rewrite attribute
                            let new_args: SystemArgs = syn::parse2(list.tokens.clone())
                                .map_err(|e| Error::new(list.span(), e).into_compile_error())?;

                            list.tokens = args.splat(&new_args).to_token_stream();
                        }
                        Meta::Path(path) => {
                            // Replace #[system] with #[system(...)]
                            attr.meta = Meta::List(MetaList { path: path.clone(), delimiter: MacroDelimiter::Paren(Paren(path.span())), tokens: args.to_token_stream()})
                        },
                        meta @ Meta::NameValue(_) => return Err(Error::new(meta.span(), "Unexpected name-value meta format").into_compile_error()),
                    }
                }
            }
            _ => ()
        }
    }

    Ok(())
}