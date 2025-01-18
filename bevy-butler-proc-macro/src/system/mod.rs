use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
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
    let transforms = &input.attr.transforms;

    let static_ident = format_ident!("_butler_{sys_ident}");

    #[cfg(not(feature="inventory"))]
    let register_block = quote! {
        #[::bevy_butler::__internal::linkme::distributed_slice(::bevy_butler::__internal::BUTLER_SLICE)]
        #[linkme(crate = ::bevy_butler::__internal::linkme)]
        static #static_ident: ::bevy_butler::__internal::ButlerRegistryEntryFactory = 
            ::bevy_butler::__internal::ButlerRegistryEntryFactory::new(
                || #plugin::_butler_sealed_marker(),
                |app| { app.add_systems(#schedule, #sys_ident #transforms ); }
            );
    };
    #[cfg(feature="inventory")]
    let register_block = quote! {
        ::bevy_butler::__internal::inventory::submit!(::bevy_butler::__internal::ButlerRegistryEntryFactory::new(
            || #plugin::_butler_sealed_marker(),
            |app| { app.add_systems(#schedule, #sys_ident #transforms ); }
        ));
    };

    Ok(quote! {
        #body

        #register_block
    })
}