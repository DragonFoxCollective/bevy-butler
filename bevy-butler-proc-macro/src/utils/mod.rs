use proc_macro2::TokenStream;
use quote::quote;
use syn::{Error, ExprClosure, Ident, Item, Path, UseTree};

pub(crate) fn butler_plugin_entry_block(
    static_ident: &Ident,
    plugin: &Path,
    expr: &ExprClosure,
) -> TokenStream {
    quote! {
        ::bevy_butler::_butler_plugin_entry!(#static_ident, ::bevy_butler::__internal::ButlerPluginRegistryEntryFactory::new(
            || #plugin::_butler_plugin_sealed_marker(),
            #expr
        ));
    }
}

pub(crate) fn butler_plugin_group_entry_block(
    static_ident: &Ident,
    plugin: &Path,
    expr: &ExprClosure,
) -> TokenStream {
    quote! {
        ::bevy_butler::_butler_plugin_group_entry!(#static_ident, ::bevy_butler::__internal::ButlerPluginGroupRegistryEntryFactory {
            type_factory: || #plugin::_butler_plugin_group_sealed_marker(),
            group_factory: #expr
        });
    }
}

pub(crate) fn get_use_path(tree: &UseTree) -> syn::Result<&Ident> {
    match tree {
        UseTree::Path(path) => get_use_path(&path.tree),
        UseTree::Name(name) => Ok(&name.ident),
        UseTree::Rename(rename) => Ok(&rename.rename),
        UseTree::Group(_) | UseTree::Glob(_) => {
            Err(syn::Error::new_spanned(tree, "Expected a path"))
        }
    }
}

pub(crate) fn get_struct_or_enum_ident(item: &Item) -> syn::Result<&Ident> {
    match item {
        Item::Struct(i) => Ok(&i.ident),
        Item::Enum(i) => Ok(&i.ident),
        Item::Use(i) => get_use_path(&i.tree),
        Item::Type(i) => Ok(&i.ident),
        other => Err(Error::new_spanned(other, "Expected a struct, enum, type alias or use statement")),
    }
}

pub(crate) fn get_fn_ident(item: &Item) -> syn::Result<&Ident> {
    match item {
        Item::Fn(i) => Ok(&i.sig.ident),
        Item::Use(i) => get_use_path(&i.tree),
        other => Err(Error::new_spanned(other, "Expected a function or use statement")),
    }
}
