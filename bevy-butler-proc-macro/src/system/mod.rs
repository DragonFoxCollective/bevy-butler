use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;
use structs::{SystemAttr, SystemInput};
use syn::parse::{Parse, Parser};

pub mod structs;

pub(crate) fn macro_impl(attr: TokenStream1, item: TokenStream1) -> syn::Result<TokenStream2> {
    let attr = SystemAttr::parse.parse(attr)?;
    let input = SystemInput::parse_with_attr(attr).parse(item)?;

    // TODO
    Ok(input.body.to_token_stream())
}