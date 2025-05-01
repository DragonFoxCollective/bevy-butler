use deluxe::ParseMetaItem;
use proc_macro2::Span;
use syn::{AngleBracketedGenericArguments, Expr, Path};

use crate::add_plugin::structs::ButlerTarget;

fn validate(mut input: AddPluginGroupAttr) -> deluxe::Result<AddPluginGroupAttr> {
    match (input._to_plugin.take(), input._to_group.take()) {
        (Some(plugin), None) => input.target = ButlerTarget::Plugin(plugin),
        (None, Some(group)) => input.target = ButlerTarget::PluginGroup(group),
        (Some(_), Some(g)) => {
            return Err(deluxe::Error::new_spanned(
                g,
                "`to_group` and `to_plugin` are mutually exclusive",
            ))
        }
        (None, None) => {
            return Err(deluxe::Error::new(
                Span::call_site(),
                "Expected `to_group` or `to_plugin`",
            ))
        }
    }

    Ok(input)
}

#[derive(ParseMetaItem)]
#[deluxe(and_then = validate)]
pub struct AddPluginGroupAttr {
    #[deluxe(rename = to_group)]
    _to_group: Option<Path>,
    #[deluxe(rename = to_plugin)]
    _to_plugin: Option<Path>,
    #[deluxe(skip)]
    #[deluxe(default = ButlerTarget::Plugin(Path { segments: Default::default(), leading_colon: None }))]
    pub target: ButlerTarget,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub init: Option<Expr>,
}
