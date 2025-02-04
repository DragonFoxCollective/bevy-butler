use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::{AddToGroupAttr, AddType};
use syn::{Error, Item};

use crate::utils::get_use_path;

pub(crate) mod structs;

pub(crate) fn macro_impl(attr: TokenStream1, body: TokenStream1) -> syn::Result<TokenStream2> {
    let attr: AddToGroupAttr = syn::parse(attr)?;
    let item: Item = syn::parse(body)?;
    let ident = match &item {
        Item::Struct(i_struct) => &i_struct.ident,
        Item::Use(i_use) => get_use_path(&i_use.tree)?,
        Item::Enum(i_enum) => &i_enum.ident,
        other => return Err(Error::new_spanned(other, "Expected `struct`, `use` or `enum`")),
    };

    let group = attr.require_group()?;

    let arghash = sha256::digest(
        ident.to_string() +
        &group.to_token_stream().to_string()
    );

    let struct_ident = format_ident!("_butler_plugin_{}_{}", ident, arghash);

    let expr = match &attr.add_type {
        Some(AddType::Before(other_plugin)) => quote! {
            |builder| builder.add_before::<#other_plugin>(#ident)
        },
        Some(AddType::After(other_plugin)) => quote! {
            |builder| builder.add_after::<#other_plugin>(#ident)
        },
        Some(AddType::Group) => quote! {
            |builder| builder.add_group(#ident)
        },
        None => quote! {
            |builder| builder.add(#ident)
        }
    };

    Ok(quote! {
        #item

        ::bevy_butler::_butler_plugin_group_entry!(#struct_ident, ::bevy_butler::__internal::ButlerPluginGroupRegistryEntryFactory {
            type_factory: || {
                let marker = #group::_butler_sealed_marker();
                marker
            },
            group_factory: #expr
        });
    })
}
