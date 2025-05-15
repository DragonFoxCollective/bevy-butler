use std::borrow::Borrow;

use deluxe::{ParseMetaItem, ParseMetaRest};
use deluxe_core::parse_helpers::skip_meta_item;
use quote::quote;
use syn::parse::discouraged::AnyDelimiter;
use syn::parse::{Parse, ParseBuffer};
use syn::punctuated::Punctuated;
use syn::Expr;
use syn::{AngleBracketedGenericArguments, ExprCall, Path, Token};

#[derive(Clone)]
pub(crate) struct TransformList(pub Vec<ExprCall>);

fn parse_end_comma_or_eof(input: &ParseBuffer<'_>) -> deluxe::Result<()> {
    if input.is_empty() {
        return Ok(());
    }

    input.parse::<Token![,]>()?;
    Ok(())
}

impl ParseMetaRest for TransformList {
    fn parse_meta_rest<'s, S: Borrow<ParseBuffer<'s>>>(
        inputs: &[S],
        exclude: &[&str],
    ) -> deluxe::Result<Self> {
        let mut ret = Vec::new();

        for input in inputs.iter() {
            let input = input.borrow();
            while !input.is_empty() {
                let path = Path::parse(input)?;
                if path
                    .get_ident()
                    .is_some_and(|i| exclude.contains(&i.to_string().as_str()))
                {
                    skip_meta_item(input);
                    parse_end_comma_or_eof(input)?;
                    continue;
                }

                // Style 1: Path - transform
                if input.peek(Token![,]) {
                    ret.push(syn::parse2(quote!(#path () ))?);
                    parse_end_comma_or_eof(input)?;
                    continue;
                }

                // Style 2: NameValue - transform = expr
                if input.peek(Token![=]) {
                    input.parse::<Token![=]>()?;
                    let expr: Expr = Expr::parse(input)?;
                    let trns: ExprCall = syn::parse2(quote!(#path ( #expr )))?;
                    ret.push(trns);

                    parse_end_comma_or_eof(input)?;
                    continue;
                }

                // Style 3: List - transform(expr1, expr2)
                let (_, _, inner) = input.parse_any_delimiter()?;
                let args = Punctuated::<Expr, Token![,]>::parse_terminated(&inner)?;
                ret.push(syn::parse2(quote!(#path (#args)))?);
                parse_end_comma_or_eof(input)?;
            }
        }
        Ok(TransformList(ret))
    }
}

#[derive(Clone, ParseMetaItem)]
pub(crate) struct SystemAttr {
    pub plugin: Path,
    pub schedule: Expr,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub pipe_in: Option<Vec<Expr>>,
    #[deluxe(rest)]
    pub transforms: TransformList,
}
