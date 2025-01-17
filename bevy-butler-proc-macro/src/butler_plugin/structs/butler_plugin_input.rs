use proc_macro2::Span;
use syn::{parse::{ParseStream, Parser}, punctuated::Punctuated, spanned::Spanned, Error, Item, ItemImpl, ItemStruct, Token};

use super::PluginStageData;

pub(crate) struct ButlerPluginAttr {
    pub span: Span,
    pub stages: [Option<PluginStageData>; 3],
}

impl ButlerPluginAttr {
    pub fn parse_inner(input: ParseStream) -> syn::Result<Self> {
        let mut ret = ButlerPluginAttr {
            span: input.span(),
            stages: Default::default(),
        };

        for (stage, data) in Punctuated::<PluginStageData, Token![,]>::parse_terminated(input)?
            .into_iter()
            .map(|d| (d.stage, d))
        {
            if ret.stages[stage as usize].is_some() {
                return Err(Error::new(data.attr_span, format!("Multiple declarations of \"{}\"", data.stage)))
            }
            ret.stages[stage as usize] = Some(data);
        }

        Ok(ret)
    }
}

pub(crate) enum ButlerPluginInput {
    Struct {
        attr: ButlerPluginAttr,
        body: ItemStruct,
    },
    Impl {
        attr: ButlerPluginAttr,
        body: ItemImpl,
    },
}

impl ButlerPluginInput {
    pub fn parse_with_attr(attr: ButlerPluginAttr) -> impl Parser<Output = Self> {
        move |input: ParseStream| {
            match input.parse::<Item>()? {
                Item::Struct(body) => Ok(Self::Struct { attr, body }),
                Item::Impl(body) => {
                    let body_span = body.span();

                    // Check if the body has a trait
                    // We can't effectively check that the trait is actually Plugin
                    // The user might have redefined the name, so we'll just assume it's correct if present
                    if body.trait_.is_none() {
                        return Err(Error::new(body_span, "Expected an `impl Plugin` block"));
                    }
                    Ok(Self::Impl { attr, body })
                },
                item => Err(Error::new_spanned(item, "Expected a free-standing fn or an `impl Plugin` block"))
            }
        }
    }
}