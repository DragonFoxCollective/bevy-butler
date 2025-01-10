#![cfg_attr(feature = "nightly", feature(stmt_expr_attributes))]

use config_systems_impl::ConfigSystems;
use proc_macro::TokenStream;
use quote::quote;
#[cfg(feature = "nightly")]
use syn::ExprBlock;
use syn::{parse_macro_input, Error, Item, ItemFn};
use system_impl::SystemArgs;

mod utils;

mod butler_plugin_impl;
mod system_impl;

#[proc_macro_attribute]
pub fn butler_plugin(args: TokenStream, item: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(item as Item);

    match parsed {
        Item::Impl(item_impl) => butler_plugin_impl::butler_plugin_impl(args, item_impl),
        Item::Struct(item_struct) => butler_plugin_impl::butler_plugin_struct(args, item_struct),

        _ => Error::new_spanned(
            parsed,
            "#[butler_plugin] can only be invoked on structs or `impl Plugin` blocks.",
        )
        .to_compile_error()
        .into(),
    }
}

#[proc_macro_attribute]
pub fn system(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as SystemArgs);
    let item = parse_macro_input!(item as ItemFn);

    match system_impl::system_free_standing_impl(args, item) {
        Ok(tokens) | Err(tokens) => tokens.into(),
    }
}

mod config_systems_impl;
#[cfg(feature = "nightly")]
#[proc_macro_attribute]
pub fn config_systems_block(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as SystemArgs);
    let mut item = parse_macro_input!(item as ExprBlock);

    let mut config = ConfigSystems {
        args,
        stmts: item.block.stmts,
    };

    if let Err(tokens) = config_systems_impl::config_impl(&mut config) {
        return tokens.into();
    }

    let stmts = &config.stmts;
    quote! {{
        #(#stmts)*
    }}
    .into()
}

#[proc_macro]
pub fn config_systems(block: TokenStream) -> TokenStream {
    let mut args: ConfigSystems =
        match syn::parse(block.clone()).map_err(|e| e.to_compile_error().into()) {
            Ok(args) => args,
            Err(e) => return e,
        };

    if let Err(tokens) = config_systems_impl::config_impl(&mut args) {
        return tokens.into();
    }

    let stmts = &args.stmts;

    quote! {
        #( #stmts )*
    }
    .into()
}
