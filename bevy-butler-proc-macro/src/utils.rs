use proc_macro_crate::{crate_name, FoundCrate};
use syn::{parenthesized, parse::{Parse, ParseStream}, Path};

pub(crate) fn get_crate(name: &str) -> Result<Path, proc_macro_crate::Error>
{
    crate_name(name).map(|found| {
        match found {
            FoundCrate::Itself => syn::parse_str(&name.to_string().replace("-", "_")).unwrap(),
            FoundCrate::Name(actual) => syn::parse_str(&format!("::{}", actual)).unwrap(),
        }
    })
}

#[derive(Debug)]
pub(crate) struct Parenthesized<T>(pub T);

impl<T> Parse for Parenthesized<T>
    where T: Parse
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);

        Ok(Self(T::parse(&content)?))
    }
}