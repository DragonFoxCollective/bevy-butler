use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::AddPluginGroupAttr;
use syn::{parse, parse_quote, Error, ExprClosure, Fields, Item, ItemEnum, ItemStruct, ItemType, ItemUse};

use crate::{add_plugin::structs::ButlerTarget, utils::{butler_plugin_entry_block, butler_plugin_group_entry_block, get_use_path}};

pub(crate) mod structs;

pub(crate) fn macro_impl(attr: TokenStream1, body: TokenStream1) -> syn::Result<TokenStream2> {
    let mut attr: AddPluginGroupAttr = parse(attr)?;
    let item: Item = parse(body)?;

    let plugin_ident = match &item {
        Item::Struct(ItemStruct { ident, fields, .. }) => {
            if attr.init.is_none() {
                if fields.is_empty() {
                    // Unit structs can be initialized using themselves
                    match fields {
                        Fields::Unit => attr.insert_init(parse_quote!(#ident))?,
                        Fields::Named(_) => attr.insert_init(parse_quote!(#ident {}))?,
                        Fields::Unnamed(_) => attr.insert_init(parse_quote!(#ident ()))?,
                    }
                }
                else {
                    attr.insert_init(parse_quote!(core::default::Default::default()))?
                }
            }
            ident
        },
        Item::Enum(ItemEnum { ident, .. }) => ident,
        Item::Use(ItemUse { tree, .. }) => get_use_path(&tree)?,
        Item::Type(ItemType { ident, .. }) => ident,
        item => return Err(Error::new_spanned(
            item,
            "Expected a struct, enum, use statement or type alias",
        ))
    };

    let generics = &attr.generics;
    let generics_without_colons = generics.clone().map(|mut g| { g.colon2_token = None; g });
    let init = &attr.init;

    let static_ident = format_ident!("_butler_add_plugin_group_{}", sha256::digest([
        plugin_ident.to_token_stream().to_string(),
        attr.require_target()?.to_string(),
        generics.to_token_stream().to_string(),
    ].concat()));

    let register_block = match attr.require_target()? {
        ButlerTarget::Plugin(plugin) => {
            let register: ExprClosure = parse_quote! { |app| {
                let plugin: #plugin_ident #generics_without_colons = {#init}.into();
                app.add_plugins(plugin);
            }};

            butler_plugin_entry_block(&static_ident, plugin, &register)
        },
        ButlerTarget::PluginGroup(group) => {
            let register = parse_quote! { |builder| {
                let group: #plugin_ident #generics_without_colons = {#init}.into();
                builder.add_group(#group)
            }};

            butler_plugin_group_entry_block(&static_ident, group, &register)
        },
    };
    
    Ok(quote! {
        #item

        #register_block
    })
}