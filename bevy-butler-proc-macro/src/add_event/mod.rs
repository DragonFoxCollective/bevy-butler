use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::*;
use syn::Item;

use crate::utils::{butler_plugin_entry_block, get_struct_or_enum_ident};

pub(crate) mod structs;

pub(crate) fn macro_impl(attr: TokenStream1, body: TokenStream1) -> syn::Result<TokenStream2> {
    let attr: EventAttr = deluxe::parse(attr)?;
    let item = syn::parse::<Item>(body)?;
    let event_ident = get_struct_or_enum_ident(&item)?;

    let plugin = &attr.plugin;
    let generics = &attr.generics;

    let mut hash_bytes = event_ident.to_string();
    hash_bytes += &plugin.to_token_stream().to_string();
    hash_bytes += &generics.to_token_stream().to_string();
    let static_ident = format_ident!("_butler_event_{}", sha256::digest(hash_bytes));

    let register_block = butler_plugin_entry_block(
        &static_ident,
        &attr.plugin,
        &syn::parse_quote! {
            |app| { app.add_event::<#event_ident #generics>(); }
        },
    );

    Ok(quote! {
        #item

        #register_block
    })
}
