use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::ObserverAttr;
use syn::{Error, Expr, Ident, Item};

use crate::utils::{butler_plugin_entry_block, get_use_path};

pub(crate) mod structs;

pub(crate) fn parse_observer(attr: &ObserverAttr, ident: &Ident) -> syn::Result<Expr> {
    let generics = &attr.generics;
    syn::parse2(quote!(#ident #generics))
}

pub(crate) fn macro_impl(attr: TokenStream1, body: TokenStream1) -> syn::Result<TokenStream2> {
    let attr: ObserverAttr = deluxe::parse(attr)?;
    let item = syn::parse::<Item>(body)?;
    let ident = match &item {
        Item::Fn(item_fn) => &item_fn.sig.ident,
        Item::Use(item_use) => get_use_path(&item_use.tree)?,
        item => return Err(Error::new_spanned(item, "Expected an `fn` or `use` item")),
    };

    let plugin = &attr.plugin;
    let obsrv_expr = parse_observer(&attr, ident)?;

    let mut hash_bytes = "observer".to_string();
    hash_bytes += &plugin.to_token_stream().to_string();
    hash_bytes += &obsrv_expr.to_token_stream().to_string();

    let static_ident = format_ident!("_butler_observer_{}", sha256::digest(hash_bytes));

    let register_block = butler_plugin_entry_block(
        &static_ident,
        plugin,
        &syn::parse_quote! {
            |app| { app.add_observer( #obsrv_expr ); }
        },
    );

    Ok(quote! {
        #item

        #register_block
    })
}
