use deluxe::ParseMetaItem;
use proc_macro2::Span;
use syn::{parse_quote, AngleBracketedGenericArguments, Expr, ExprClosure, Ident, Path, TypePath};

/// Whether to add to a `Plugin` or a `PluginGroup`.
#[derive(Debug, Clone)]
pub(crate) enum ButlerTarget {
    Plugin(Path),
    PluginGroup(Path),
}

impl ToString for ButlerTarget {
    fn to_string(&self) -> String {
        match self {
            Self::Plugin(p) => format!("Plugin({p:?})"),
            Self::PluginGroup(g) => format!("PluginGroup({g:?})"),
        }
    }
}

fn validate(mut input: AddPluginAttr) -> deluxe::Result<AddPluginAttr> {
    match (input._to_plugin.take(), input._to_group.take()) {
        (Some(plugin), None) => input.target = ButlerTarget::Plugin(plugin),
        (None, Some(group)) => input.target = ButlerTarget::PluginGroup(group),
        (Some(_), Some(g)) => return Err(deluxe::Error::new_spanned(g, "`to_group` and `to_plugin` are mutually exclusive")),
        (None, None) => return Err(deluxe::Error::new(Span::call_site(), "Expected `to_group` or `to_plugin`")),
    }

    Ok(input)
}

#[derive(ParseMetaItem)]
#[deluxe(and_then = validate)]
pub(crate) struct AddPluginAttr {
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

impl AddPluginAttr {
    /// Returns the statement that will be used to register this Plugin to the target
    pub fn register_statement(&self, plugin: &Ident) -> syn::Result<ExprClosure> {
        let target = &self.target;
        let generics = &self.generics;
        let generics_without_colons = generics.clone().map(|mut g| { g.colon2_token = None; return g;});
        let init = self.init.as_ref().cloned().unwrap_or_else(|| parse_quote! { <#plugin #generics_without_colons as core::default::Default>::default() });

        Ok(match target {
            ButlerTarget::Plugin(_) => parse_quote! { |app| {
                let plugin: #plugin #generics_without_colons = {#init}.into();
                app.add_plugins(plugin);
            } },
            ButlerTarget::PluginGroup(_) => parse_quote! { |builder: ::bevy_butler::__internal::bevy_app::PluginGroupBuilder| -> ::bevy_butler::__internal::bevy_app::PluginGroupBuilder {
                    let plugin: #plugin #generics_without_colons = {#init}.into();
                    builder.add(plugin)
            } },
        })
    }
}
