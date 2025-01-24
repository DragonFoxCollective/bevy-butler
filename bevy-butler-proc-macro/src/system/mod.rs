use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::{SystemAttr, SystemInput};
use syn::{parse::{Parse, Parser}, Expr};

pub mod structs;

pub(crate) fn parse_system(input: &SystemInput) -> Expr {
    let body = &input.body;
    let sys_ident = &body.sig.ident;

    let transforms = Some(&input.attr.transforms).filter(|i| !i.is_empty()).into_iter();
    let generics = input.attr.generics.as_ref();

    syn::parse_quote!(#sys_ident #generics #(. #transforms)*)
}

pub(crate) fn macro_impl(attr: TokenStream1, item: TokenStream1) -> syn::Result<TokenStream2> {
    let input = SystemInput::parse_with_attr(SystemAttr::parse.parse(attr)?).parse(item)?;
    let body = &input.body;

    let plugin = input.attr.require_plugin()?;
    let schedule = input.attr.require_schedule()?;

    let sys_expr = parse_system(&input);

    let mut hash_bytes = "system".to_string();
    hash_bytes += &plugin.to_token_stream().to_string();
    hash_bytes += &schedule.to_token_stream().to_string();
    hash_bytes += &sys_expr.to_token_stream().to_string();
    #[allow(unused_variables)] // It's actually used
    let static_ident = format_ident!("_butler_system_{}", sha256::digest(hash_bytes));

    let register_block = quote! {
        ::bevy_butler::butler_entry!(#static_ident, ::bevy_butler::__internal::ButlerRegistryEntryFactory::new(
            || #plugin::_butler_sealed_marker(),
            |app| { app.add_systems( #schedule, #sys_expr ); }
        ));
    };

    Ok(quote! {
        #body

        #register_block
    })
}