#![cfg_attr(feature = "nightly", feature(stmt_expr_attributes))]

use butler_plugin_impl::{ButlerPluginInput, PluginStageData};
use config_systems_impl::ConfigSystemsInput;
use proc_macro::TokenStream;
use quote::quote;
#[cfg(feature = "nightly")]
use syn::ExprBlock;
use syn::{parse::Parser, parse_macro_input, ItemFn};
use system_impl::{SystemArgs, SystemAttr, SystemInput};
use system_set_impl::SystemSetInput;

mod utils;

mod butler_plugin_impl;
#[proc_macro_attribute]
pub fn butler_plugin(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(item as ButlerPluginInput);

    match PluginStageData::parse_as_list.parse(args) {
        Ok(data) => {
            if let Err(e) = item.stage_data().merge(data).map_err(|e| e.to_compile_error().into()) {
                return e;
            }
        },
        Err(e) => { return e.to_compile_error().into(); },
    }

    match butler_plugin_impl::macro_impl(item) {
        Ok(tokens) | Err(tokens) => tokens.into(),
    }
}

mod system_impl;
#[proc_macro_attribute]
pub fn system(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as SystemArgs);
    let item = parse_macro_input!(item as ItemFn);

    let input = SystemInput {
        attr: SystemAttr {
            span: args.span,
            args: Some(args),
        },
        item,
    };

    match system_impl::free_standing_impl(input) {
        Ok(tokens) | Err(tokens) => tokens.into(),
    }
}

mod config_systems_impl;
#[cfg(feature = "nightly")]
#[proc_macro_attribute]
pub fn config_systems_block(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as SystemArgs);
    let item = parse_macro_input!(item as ExprBlock);

    let mut config = ConfigSystemsInput {
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
    let mut args: ConfigSystemsInput =
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

mod system_set_impl;
#[proc_macro]
pub fn system_set(block: TokenStream) -> TokenStream {
    let input: SystemSetInput = parse_macro_input!(block as SystemSetInput);

    match system_set_impl::macro_impl(input) {
        Ok(tokens) | Err(tokens) => tokens.into(),
    }
}
