use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::ObserverAttr;
use syn::{Expr, Ident, Item};

use crate::utils::{butler_plugin_entry_block, get_fn_ident};

pub(crate) mod structs;

pub(crate) fn parse_observer(attr: &ObserverAttr, ident: &Ident) -> Expr {
    let generics = attr.generics.clone().map(|mut g| {
        g.colon2_token = Some(Default::default());
        g
    });
    syn::parse_quote! {
        #ident #generics
    }
}

pub(crate) fn macro_impl(attr: TokenStream1, body: TokenStream1) -> syn::Result<TokenStream2> {
    let attr: ObserverAttr = deluxe::parse(attr)?;
    let item = syn::parse::<Item>(body)?;
    let ident = get_fn_ident(&item)?;

    let plugin = &attr.plugin;
    let obsrv_expr = parse_observer(&attr, ident);

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
