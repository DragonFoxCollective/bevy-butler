use quote::quote;
use syn::{parse::{discouraged::Speculative, ParseStream}, punctuated::Punctuated, AngleBracketedGenericArguments, Error, Ident, Meta, Token, TypePath};

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