use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::AddPluginAttr;
use syn::{Error, Item};

use crate::utils::{butler_plugin_entry_block, get_use_path};

pub mod structs;

pub(crate) fn macro_impl(attr: TokenStream1, body: TokenStream1) -> syn::Result<TokenStream2> {
    let attr: AddPluginAttr = syn::parse(attr)?;
    let item: Item = syn::parse(body)?;
    let ident = match &item {
        Item::Struct(i_struct) => &i_struct.ident,
        Item::Use(i_use) => get_use_path(&i_use.tree)?,
        Item::Enum(i_enum) => &i_enum.ident,
        other => return Err(Error::new_spanned(other, "Expected `struct`, `use` or `enum`")),
    };

    let plugin = attr.require_plugin()?;

    let arghash = sha256::digest(
        ident.to_string() +
        &plugin.to_token_stream().to_string()
    );

    let struct_ident = format_ident!("_butler_add_plugin_{}_{}", ident, arghash);

    let register_block = butler_plugin_entry_block(
        &struct_ident,
        plugin,
        &syn::parse_quote! {
            |app| { app.add_plugins(#ident); }
        },
    );

    Ok(quote! {
        #item

        #register_block
    })
}