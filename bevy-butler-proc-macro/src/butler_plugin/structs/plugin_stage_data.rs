use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse::{Parse, ParseStream}, spanned::Spanned, Error, Expr, ExprAssign, ExprBlock, ExprCall, Ident, ImplItemFn};

use super::PluginStage;

pub(crate) struct PluginStageData {
    pub stage: PluginStage,
    pub attr_span: Span,
    pub stmts: Vec<ExprCall>,
}

fn expr_to_stage_arg(expr: Expr) -> syn::Result<ExprCall> {
    match expr {
        Expr::Path(p) => syn::parse2(quote!(#p ())),
        Expr::Call(c) => Ok(c),
        Expr::Assign(ExprAssign { left, right, .. }) => syn::parse2(quote!(#left (#right))),
        other => Err(Error::new_spanned(other, "Expected a &mut App method name")),
    }
}

impl Parse for PluginStageData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Ensure that this is a stage
        let stage = input.fork().parse::<PluginStage>()?;

        match input.parse::<Expr>()? {
            // `stage(stmt1, stmt2, ...)`
            Expr::Call(call) => {
                Ok(Self {
                    attr_span: call.span(),
                    stage,
                    stmts: Result::from_iter(call.args.into_iter().map(|expr| expr_to_stage_arg(expr)))?,
                })
            },
            // `stage = statement`
            Expr::Assign(ExprAssign { left, right, .. }) => syn::parse2(quote!(#left (#right))),
            other => Err(Error::new_spanned(other, "Expected name-value or list of arguments")),
        }
    }
}

impl PluginStageData {
    pub fn stage_inner_block(&self, app: &Ident) -> ExprBlock {
        let stmts = &self.stmts;
        syn::parse_quote!({{
            #( #app . #stmts; )*
        }})
    }

    pub fn stage_fn(&self) -> ImplItemFn {
        let stage = self.stage;
        let inner_block = self.stage_inner_block(&format_ident!("app"));

        syn::parse_quote! {
            fn #stage(&self, app: &mut ::bevy_butler::__internal::bevy_app::App) {
                #inner_block
            }
        }
    }
}