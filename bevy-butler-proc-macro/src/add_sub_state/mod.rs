use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::AddSubStateAttr;
use syn::Item;

use crate::utils::{butler_plugin_entry_block, get_struct_or_enum_ident};

pub mod structs;

pub fn macro_impl(attr: TokenStream1, body: TokenStream1) -> syn::Result<TokenStream2> {
    let attr: AddSubStateAttr = deluxe::parse(attr)?;
    let item: Item = syn::parse(body)?;
    let ident = get_struct_or_enum_ident(&item)?;
    let generics = &attr.generics;

    let static_ident = format_ident!(
        "_butler_sub_state_{}",
        sha256::digest([
            attr.plugin.to_token_stream().to_string(),
            attr.generics.to_token_stream().to_string(),
        ].concat())
    );

    let register_block = butler_plugin_entry_block(
        &static_ident,
        &attr.plugin,
        &syn::parse_quote! {
            |app| { ::bevy_butler::__internal::bevy_state::app::AppExtStates::add_sub_state::<#ident #generics>(app); }
        }
    );

    Ok(quote! {
        #item

        #register_block
    })
}