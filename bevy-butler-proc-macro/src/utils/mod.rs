use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::{discouraged::Speculative, Parse, ParseStream}, punctuated::Punctuated, AngleBracketedGenericArguments, Error, ExprClosure, Ident, Meta, Token, TypePath, UseTree};

/// Used to parse `generics = <...>`, `generics(...)` and `generics = ...`
/// Returns None if the meta identifier is not `generics`
pub(crate) fn try_parse_generics_arg(input: ParseStream) -> syn::Result<Option<AngleBracketedGenericArguments>> {
    if !input.fork().parse::<Ident>().is_ok_and(|i| i == "generics") {
        return Ok(None);
    }

    // First check for Meta-valid forms, like `generics(...) and generics = ...`
    let fork = input.fork();
    match fork.parse::<Meta>() {
        Ok(meta) => {
            input.advance_to(&fork);
            match meta {
                Meta::List(list) => {
                    let args = list.parse_args_with(Punctuated::<TypePath, Token![,]>::parse_terminated)?;
                    Ok(Some(syn::parse2(quote!(::<#args>))?))
                },
                Meta::NameValue(name_value) => {
                    let arg = name_value.value;
                    Ok(Some(syn::parse2(quote!(::<#arg>))?))
                },
                Meta::Path(p) => Err(Error::new_spanned(p, "Expected `generics = <...>`, `generics(...)` or `generics = ...`")),
            }
        }
        Err(e) => {
            // Try to parse `generics = <...>`
            let args = input.parse::<Ident>()
                .and_then(|_| input.parse::<Token![=]>())
                .and_then(|_| input.parse::<AngleBracketedGenericArguments>())
                .map_err(|_| Error::new(e.span(), "Expected `generics = <...>`, `generics(...)` or `generics = ...`"));

            Ok(Some(args?))
        }
    }
}

pub(crate) fn parse_meta_args<T: Parse>(meta: Meta) -> syn::Result<T> {
    match meta {
        Meta::List(list) => list.parse_args(),
        Meta::NameValue(name_value) => syn::parse2(name_value.value.to_token_stream()),
        Meta::Path(p) => Err(Error::new_spanned(p, "Expected parenthesis or `name = value`")),
    }
}

pub(crate) fn butler_entry_block(static_ident: &Ident, plugin: &TypePath, expr: &ExprClosure) -> TokenStream {
    quote! {
        ::bevy_butler::butler_entry!(#static_ident, ::bevy_butler::__internal::ButlerRegistryEntryFactory::new(
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
        UseTree::Group(_) | UseTree::Glob(_) => Err(syn::Error::new_spanned(tree, "Expected a path")),
    }
}

pub(crate) enum GenericOrMeta {
    Generic(AngleBracketedGenericArguments),
    Meta(Meta),
}

impl Parse for GenericOrMeta {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if let Some(generics) = try_parse_generics_arg(input)? {
            Ok(GenericOrMeta::Generic(generics))
        }
        else {
            Ok(GenericOrMeta::Meta(input.parse()?))
        }
    }
}