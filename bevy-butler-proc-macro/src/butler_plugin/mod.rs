use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use structs::{ButlerPluginAttr, ButlerPluginInput};
use syn::{parse::Parser, ItemStruct};

pub mod structs;

pub(crate) fn macro_impl(attr: TokenStream1, item: TokenStream1) -> syn::Result<TokenStream2> {
    let attr = ButlerPluginAttr::parse_inner.parse(attr)?;
    let input = ButlerPluginInput::parse_with_attr(attr).parse(item)?;

    match input {
        ButlerPluginInput::Struct { attr, body } => struct_impl(attr, body),
        ButlerPluginInput::Impl { attr, body } => todo!(),
    }
}

pub(crate) fn struct_impl(attr: ButlerPluginAttr, body: ItemStruct) -> syn::Result<TokenStream2> {
    let plugin_struct = &body.ident;
    Ok(quote! {
        #body

        // TODO

        impl ::bevy_butler::__internal::bevy_app::Plugin for #plugin_struct {
            fn build(&self, app: &mut ::bevy_butler::__internal::bevy_app::App) {

            }
        }
    })
}