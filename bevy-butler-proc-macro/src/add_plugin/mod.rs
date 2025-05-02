use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::{AddPluginAttr, ButlerTarget};
use syn::{parse, parse_quote, Fields, Item, ItemStruct};

use crate::utils::{butler_plugin_entry_block, butler_plugin_group_entry_block, get_struct_or_enum_ident};

pub mod structs;

pub(crate) fn macro_impl(attr: TokenStream1, body: TokenStream1) -> syn::Result<TokenStream2> {
    let mut attr: AddPluginAttr = deluxe::parse(attr)?;
    let item: Item = parse(body)?;
    let plugin_ident = get_struct_or_enum_ident(&item)?;

    if let Item::Struct(ItemStruct { ident, fields, .. }) = &item {
        if attr.init.is_none() && fields.is_empty() {
            // Unit structs can be initialized using themselves
            match fields {
                Fields::Unit => attr.init = Some(parse_quote!(#ident)),
                Fields::Named(_) => attr.init = Some(parse_quote!(#ident {})),
                Fields::Unnamed(_) => attr.init = Some(parse_quote!(#ident ())),
            }
        }
    }

    let generics = &attr.generics;

    let register = attr.register_statement(plugin_ident)?;

    let hash = sha256::digest(
        [
            plugin_ident.to_token_stream().to_string(),
            attr.target.to_string(),
            generics.to_token_stream().to_string(),
        ]
        .concat(),
    );
    let static_ident = format_ident!("_butler_add_plugin_{}", hash);

    let register_block = match attr.target {
        ButlerTarget::Plugin(target) => {
            butler_plugin_entry_block(&static_ident, &target, &register)
        }
        ButlerTarget::PluginGroup(group) => {
            butler_plugin_group_entry_block(&static_ident, &group, &register)
        }
    };

    Ok(quote! {
        #item

        #register_block
    })
}
