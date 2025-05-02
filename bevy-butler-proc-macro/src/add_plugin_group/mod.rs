use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::AddPluginGroupAttr;
use syn::{
    parse, parse_quote, ExprClosure, Fields, Item, ItemStruct,
};

use crate::{
    add_plugin::structs::ButlerTarget,
    utils::{butler_plugin_entry_block, butler_plugin_group_entry_block, get_struct_or_enum_ident},
};

pub(crate) mod structs;

pub(crate) fn macro_impl(attr: TokenStream1, body: TokenStream1) -> syn::Result<TokenStream2> {
    let mut attr: AddPluginGroupAttr = deluxe::parse(attr)?;
    let item: Item = parse(body)?;

    let plugin_ident = get_struct_or_enum_ident(&item)?;

    if let Item::Struct(ItemStruct { fields, ident, .. }) = &item {
        if attr.init.is_none() {
            if fields.is_empty() {
                // Unit structs can be initialized using themselves
                match fields {
                    Fields::Unit => attr.init = Some(parse_quote!(#ident)),
                    Fields::Named(_) => attr.init = Some(parse_quote!(#ident {})),
                    Fields::Unnamed(_) => attr.init = Some(parse_quote!(#ident ())),
                }
            }
        }
    }

    if attr.init.is_none() {
        attr.init = Some(parse_quote!(core::default::Default::default()));
    }

    let generics = &attr.generics;
    let generics_without_colons = generics.clone().map(|mut g| {
        g.colon2_token = None;
        g
    });
    let init = &attr.init;

    let static_ident = format_ident!(
        "_butler_add_plugin_group_{}",
        sha256::digest(
            [
                plugin_ident.to_token_stream().to_string(),
                attr.target.to_string(),
                generics.to_token_stream().to_string(),
            ]
            .concat()
        )
    );

    let register_block = match attr.target {
        ButlerTarget::Plugin(target) => {
            let register: ExprClosure = parse_quote! { |app| {
                let plugin: #plugin_ident #generics_without_colons = {#init};
                app.add_plugins(plugin);
            }};

            butler_plugin_entry_block(&static_ident, &target, &register)
        }
        ButlerTarget::PluginGroup(target) => {
            let register = parse_quote! { |builder| {
                let group: #plugin_ident #generics_without_colons = {#init};
                builder.add_group(group)
            }};

            butler_plugin_group_entry_block(&static_ident, &target, &register)
        }
    };

    Ok(quote! {
        #item

        #register_block
    })
}
