use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::{discouraged::Speculative, Parse, ParseStream, Parser}, punctuated::Punctuated, AngleBracketedGenericArguments, Error, ExprClosure, Ident, ImplGenerics, Meta, Path, Token, TypeGenerics, TypePath, UseTree
};

pub(crate) fn butler_plugin_entry_block(
    static_ident: &Ident,
    plugin: &Path,
    expr: &ExprClosure,
) -> TokenStream {
    quote! {
        ::bevy_butler::_butler_plugin_entry!(#static_ident, ::bevy_butler::__internal::ButlerPluginRegistryEntryFactory::new(
            || #plugin::_butler_sealed_marker(),
            #expr
        ));
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
