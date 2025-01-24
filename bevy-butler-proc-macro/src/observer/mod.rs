use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::{ObserverAttr, ObserverInput};
use syn::{parse::{Parse, Parser}, Expr, ItemFn};

pub(crate) mod structs;

pub(crate) fn parse_observer(input: &ObserverInput) -> syn::Result<Expr> {
    let ident = &input.func.sig.ident;
    let generics = &input.attr.generics;
    syn::parse2(quote!(#ident #generics))
}

pub(crate) fn macro_impl(attr: TokenStream1, body: TokenStream1) -> syn::Result<TokenStream2> {
    let input = ObserverInput {
        attr: ObserverAttr::parse.parse(attr)?,
        func: ItemFn::parse.parse(body)?,
    };

    let plugin = input.attr.require_plugin()?;
    let obsrv_expr = parse_observer(&input)?;

    let mut hash_bytes = "observer".to_string();
    hash_bytes += &plugin.to_token_stream().to_string();
    hash_bytes += &obsrv_expr.to_token_stream().to_string();

    let static_ident = format_ident!("_butler_observer_{}", sha256::digest(hash_bytes));

    let register_block = quote! {
        ::bevy_butler::butler_entry!(#static_ident, ::bevy_butler::__internal::ButlerRegistryEntryFactory::new(
            || #plugin::_butler_sealed_marker(),
            |app| { app.add_observer( #obsrv_expr ); }
        ));
    };

    let body = input.func;

    Ok(quote! {
        #body

        #register_block
    })
}