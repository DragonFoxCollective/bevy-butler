use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use structs::{ButlerPluginAttr, ButlerPluginInput};
use syn::{parse::Parser, ItemStruct};

mod structs;

pub(crate) fn macro_impl(attr: TokenStream1, item: TokenStream1) -> syn::Result<TokenStream2> {
    eprintln!("TOKENS: {}", attr.to_string());
    let attr = ButlerPluginAttr::parse_inner.parse(attr)?;
    let input = ButlerPluginInput::parse_with_attr(attr).parse(item)?;

    match input {
        ButlerPluginInput::Struct { attr, body } => struct_impl(attr, body),
        ButlerPluginInput::Impl { attr, body } => todo!(),
    }
}

pub(crate) fn struct_impl(attr: ButlerPluginAttr, body: ItemStruct) -> syn::Result<TokenStream2> {
    // TODO
    Ok(body.to_token_stream())
}