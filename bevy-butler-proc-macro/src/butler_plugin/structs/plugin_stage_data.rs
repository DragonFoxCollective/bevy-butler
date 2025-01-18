use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{parse::{Parse, ParseStream, Parser}, punctuated::Punctuated, spanned::Spanned, Error, Expr, ExprBlock, ExprCall, Ident, ImplItemFn, Meta, Token};

use super::PluginStage;

pub(crate) struct PluginStageData {
    pub stage: PluginStage,
    pub attr_span: Span,
    pub stmts: Vec<ExprCall>,
}

impl TryFrom<Meta> for PluginStageData {
    type Error = syn::Error;

    fn try_from(value: Meta) -> Result<Self, Self::Error> {
        let stage = PluginStage::try_from(value.path())?;
        match value {
            Meta::NameValue(name_value) => {
                let attr_span = name_value.span();
                let expr = match name_value.value {
                    Expr::Call(call) => call,
                    Expr::Path(path) => syn::parse_quote!(#path ()),
                    expr => return Err(Error::new_spanned(expr, "Expected system transform function")),
                };
                Ok(PluginStageData {
                    attr_span,
                    stage,
                    stmts: vec![expr],
                })
            }
            Meta::List(list) => {
                let attr_span = list.span();
                let stmts = Punctuated::<ExprCall, Token![,]>::parse_terminated.parse2(list.tokens)?;
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

impl PluginStageData {
    pub fn stage_inner_block(&self, app: Ident) -> ExprBlock {
        let stmts = &self.stmts;
        syn::parse_quote!({{
            #( #app.#stmts; )*
        }})
    }

    pub fn stage_fn(&self) -> ImplItemFn {
        let stage = self.stage;
        let inner_block = self.stage_inner_block(syn::parse_quote!(app));
        syn::parse_quote! {
            fn #stage(&self, app: &mut ::bevy_butler::__internal::bevy_app::App) {
                #inner_block
            }
        }
    }
}