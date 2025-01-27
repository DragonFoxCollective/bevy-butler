use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::{SystemAttr, SystemInput};
use syn::Ident;
use syn::{
    parse::{Parse, Parser},
    Expr,
};

use crate::utils::{butler_entry_block, get_use_path};

pub mod structs;

pub(crate) fn parse_system(attr: &SystemAttr, ident: &Ident) -> Expr {
    let transforms = Some(&attr.transforms).filter(|i| !i.is_empty()).into_iter();
    let generics = attr.generics.as_ref();

    syn::parse_quote!(#ident #generics #(. #transforms)*)
}

pub(crate) fn macro_impl(attr: TokenStream1, item: TokenStream1) -> syn::Result<TokenStream2> {
    let attr = SystemAttr::parse.parse(attr)?;
    let input = SystemInput::parse_with_attr(attr).parse(item)?;
    let (attr, ident) = match &input {
        SystemInput::Fn { attr, body } => (attr, &body.sig.ident),
        SystemInput::Use { attr, body } => (attr, get_use_path(&body.tree)?),
    };

    let plugin = attr.require_plugin()?;
    let schedule = attr.require_schedule()?;

    let sys_expr = parse_system(attr, ident);

    let mut hash_bytes = "system".to_string();
    hash_bytes += &plugin.to_token_stream().to_string();
    hash_bytes += &schedule.to_token_stream().to_string();
    hash_bytes += &sys_expr.to_token_stream().to_string();
    #[allow(unused_variables)] // It's actually used
    let static_ident = format_ident!("_butler_system_{}", sha256::digest(hash_bytes));

    let register_block = butler_entry_block(
        &static_ident,
        plugin,
        &syn::parse_quote! {
            |app| { app.add_systems( #schedule, #sys_expr ); }
        },
    );

    match input {
        SystemInput::Fn { body, .. } => Ok(quote! {
            #body

            #register_block
        }),
        SystemInput::Use { body, .. } => Ok(quote! {
            #body

            #register_block
        }),
    }
}
