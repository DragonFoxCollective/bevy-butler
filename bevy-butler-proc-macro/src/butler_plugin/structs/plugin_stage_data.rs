use proc_macro2::Span;
use syn::{parse::{Parse, ParseStream, Parser}, punctuated::Punctuated, spanned::Spanned, Error, Expr, Meta, Token};

use super::PluginStage;

pub(crate) struct PluginStageData {
    pub stage: PluginStage,
    pub attr_span: Span,
    pub stmts: Vec<Expr>,
}

impl TryFrom<Meta> for PluginStageData {
    type Error = syn::Error;

    fn try_from(value: Meta) -> Result<Self, Self::Error> {
        let stage = PluginStage::try_from(value.path())?;
        match value {
            Meta::NameValue(name_value) => {
                Ok(PluginStageData {
                    attr_span: name_value.span(),
                    stage,
                    stmts: vec![name_value.value],
                })
            }
            Meta::List(list) => {
                let attr_span = list.span();
                let stmts = Punctuated::<Expr, Token![,]>::parse_terminated.parse2(list.tokens)?;
                Ok(PluginStageData {
                    attr_span,
                    stage,
                    stmts: stmts.into_iter().collect(),
                })
            }
            Meta::Path(_) => Err(Error::new_spanned(value, "Expected name-value or list of expressions")),
        }
    }
}

impl Parse for PluginStageData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Self::try_from(input.parse::<Meta>()?)
    }
}