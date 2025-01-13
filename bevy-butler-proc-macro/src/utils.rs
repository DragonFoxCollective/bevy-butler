use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
};

#[derive(Debug)]
pub(crate) struct Parenthesized<T>(pub T);

impl<T> Parse for Parenthesized<T>
where
    T: Parse,
{
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);

        Ok(Self(T::parse(&content)?))
    }
}
