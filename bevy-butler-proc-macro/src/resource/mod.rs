use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::*;
use syn::{
    parse::{Parse, Parser},
    Error, Item,
};

use crate::utils::{butler_entry_block, get_use_path};

pub(crate) mod structs;

pub(crate) fn macro_impl(attr: TokenStream1, body: TokenStream1) -> syn::Result<TokenStream2> {
    let attr = ResourceAttr::parse.parse(attr)?;
    let item = syn::parse::<Item>(body)?;
    let res_ident = match &item {
        Item::Struct(i_struct) => &i_struct.ident,
        Item::Use(i_use) => get_use_path(&i_use.tree)?,
        Item::Type(i_type) => &i_type.ident,
        Item::Enum(i_enum) => &i_enum.ident,
        item => {
            return Err(Error::new_spanned(
                item,
                "Expected a struct or use statement",
            ))
        }
    };

    let plugin = &attr.plugin;
    let generics = &attr.generics;

    let mut hash_bytes = res_ident.to_string();
    hash_bytes += &plugin.to_token_stream().to_string();
    hash_bytes += &generics.to_token_stream().to_string();
    let static_ident = format_ident!("_butler_resource_{}", sha256::digest(hash_bytes));

    let entry_expr = match (&attr.init, attr.non_send.unwrap_or_default()) {
        (Some(expr), false) => syn::parse_quote! {
            |app| { app.insert_resource(#expr); }
        },
        (Some(expr), true) => syn::parse_quote! {
            |app| { app.insert_non_send_resource(#expr); }
        },
        (None, false) => syn::parse_quote! {
            |app| { app.init_resource::<#res_ident #generics>(); }
        },
        (None, true) => syn::parse_quote! {
            |app| { app.init_non_send_resource::<#res_ident #generics>(); }
        },
    };

    let register_block = butler_entry_block(&static_ident, attr.require_plugin()?, &entry_expr);

    Ok(quote! {
        #item

        #register_block
    })
}
