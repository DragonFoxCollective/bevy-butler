use proc_macro2::TokenStream;
use quote::quote;
use syn::{ExprClosure, Ident, Path, UseTree};

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
