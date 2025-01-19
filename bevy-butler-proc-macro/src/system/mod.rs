use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::{SystemAttr, SystemInput};
use syn::parse::{Parse, Parser};

pub mod structs;

pub(crate) fn macro_impl(attr: TokenStream1, item: TokenStream1) -> syn::Result<TokenStream2> {
    let attr = SystemAttr::parse.parse(attr)?;
    let input = SystemInput::parse_with_attr(attr).parse(item)?;

    let body = &input.body;
    let sys_ident = &body.sig.ident;

    let plugin = input.attr.require_plugin()?;
    let schedule = input.attr.require_schedule()?;
    let transforms = input.attr.transforms.iter();
    let generics = input.attr.generics.as_ref();

    let mut hash_bytes = String::new();
    hash_bytes += &sys_ident.to_string();
    hash_bytes += &plugin.to_token_stream().to_string();
    hash_bytes += &schedule.to_token_stream().to_string();
    hash_bytes += &generics.map(|g| g.to_token_stream().to_string()).unwrap_or_default();
    hash_bytes += &transforms.clone().fold(String::new(), |bytes, t| bytes + &t.to_token_stream().to_string());
    #[allow(unused_variables)] // It's actually used
    let static_ident = format_ident!("_butler_sys_{}", sha256::digest(hash_bytes));

    let transformed_system = quote!(#sys_ident #generics #(.#transforms)*);

    let register_block = quote! {
        ::bevy_butler::butler_entry!(#static_ident, ::bevy_butler::__internal::ButlerRegistryEntryFactory::new(
            || #plugin::_butler_sealed_marker(),
            |app| { app.add_systems( #schedule, #transformed_system ); }
        ));
    };

    Ok(quote! {
        #body

        #register_block
    })
}