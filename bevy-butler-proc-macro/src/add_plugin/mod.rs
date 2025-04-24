use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::{AddPluginAttr, ButlerTarget};
use syn::{parse, parse_quote, Error, Fields, Item, ItemEnum, ItemStruct, ItemType, ItemUse};

use crate::utils::{butler_plugin_entry_block, butler_plugin_group_entry_block, get_use_path};

pub mod structs;

pub(crate) fn macro_impl(attr: TokenStream1, body: TokenStream1) -> syn::Result<TokenStream2> {
    let mut attr: AddPluginAttr = parse(attr)?;
    let item: Item = parse(body)?;
    let plugin_ident = match &item {
        Item::Struct(ItemStruct { ident, fields, .. }) => {
            if attr.init.is_none() && fields.is_empty() {
                // Unit structs can be initialized using themselves
                match fields {
                    Fields::Unit => attr.insert_init(parse_quote!(#ident))?,
                    Fields::Named(_) => attr.insert_init(parse_quote!(#ident {}))?,
                    Fields::Unnamed(_) => attr.insert_init(parse_quote!(#ident ()))?,
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

    let register = attr.register_statement(plugin_ident)?;

    let hash = sha256::digest([
        plugin_ident.to_token_stream().to_string(),
        attr.require_target()?.to_string(),
        generics.to_token_stream().to_string(),
    ].concat());
    let static_ident = format_ident!("_butler_add_plugin_{}", hash);

    let register_block = match attr.require_target()? {
        ButlerTarget::Plugin(target) => {
            butler_plugin_entry_block(&static_ident, &target, &register)
        },
        ButlerTarget::PluginGroup(group) => {
            butler_plugin_group_entry_block(&static_ident, &group, &register)
        },
    };

    Ok(quote! {
        #item

        #register_block
    })
}
