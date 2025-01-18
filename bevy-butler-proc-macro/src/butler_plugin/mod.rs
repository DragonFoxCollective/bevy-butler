use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use structs::{ButlerPluginAttr, ButlerPluginInput, PluginStage};
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

pub(crate) fn struct_impl(mut attr: ButlerPluginAttr, body: ItemStruct) -> syn::Result<TokenStream2> {
    let plugin_struct = &body.ident;
    let build_body = attr.stages[PluginStage::Build as usize].take()
        .map(|data| data.stage_inner_block(syn::parse_quote!(app)));
    let fn_iter = attr.stages.into_iter()
        .filter_map(|a| a);
    let marker_name = format_ident!("{plugin_struct}Marker");
    Ok(quote! {
        #body

        pub(crate) struct #marker_name;

        impl ::bevy_butler::__internal::bevy_app::Plugin for #plugin_struct {
            fn build(&self, app: &mut ::bevy_butler::__internal::bevy_app::App) {
                #build_body
            }
        }

        impl ::bevy_butler::ButlerPlugin for #plugin_struct {
            type Marker = #marker_name;
        }
    })
}