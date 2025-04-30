use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use structs::RegisterTypeAttr;
use syn::{
    parse::{Parse, Parser},
    Error, Item,
};

use crate::utils::{butler_plugin_entry_block, get_use_path};

pub(crate) mod structs;

pub(crate) fn macro_impl(attr: TokenStream1, body: TokenStream1) -> syn::Result<TokenStream2> {
    let attr: RegisterTypeAttr = deluxe::parse(attr)?;
    let item: Item = syn::parse(body)?;
    let type_ident = match &item {
        Item::Struct(i_struct) => &i_struct.ident,
        Item::Use(i_use) => get_use_path(&i_use.tree)?,
        Item::Type(i_type) => &i_type.ident,
        Item::Enum(i_enum) => &i_enum.ident,
        item => {
            return Err(Error::new_spanned(
                item,
                "Expected a `struct`, `use`, `enum` or `type` item",
            ))
        }
    };

    let plugin = &attr.plugin;
    let type_data = &attr.type_data;

    let static_ident = format_ident!("_butler_typereg_{}", sha256::digest(type_ident.to_string()));
    let entry_expr = syn::parse_quote! {
        |app| {
            app.register_type::<#type_ident>()#(
                .register_type_data::<#type_ident, #type_data>())*;
        }
    };

    let register_block = butler_plugin_entry_block(&static_ident, plugin, &entry_expr);

    Ok(quote! {
        #item

        #register_block
    })
}
