use proc_macro2::Span;
use syn::{parse::{Parse, ParseStream}, parse_quote, AngleBracketedGenericArguments, Error, Expr, ExprClosure, Ident, Token, TypePath};

use crate::utils::{parse_meta_args, GenericOrMeta};

/// When adding to a `PluginGroup`, this will either
/// add using `PluginGroupBuilder.add_before` or
/// `PluginGroupBuilder.add_after`.
#[derive(Debug, Clone)]
pub(crate) enum GroupOrdering {
    Before(TypePath),
    After(TypePath),
}

impl GroupOrdering {
    pub fn get_error(order1: &Self, order2: &Self) -> Error {
        match (order1, order2) {
            (GroupOrdering::Before(_), GroupOrdering::Before(_)) => Error::new(Span::call_site(), "Multiple declarations of \"before\""),
            (GroupOrdering::After(_), GroupOrdering::After(_)) => Error::new(Span::call_site(), "Multiple declarations of \"after\""),
            _ => Error::new(Span::call_site(), "\"before\" and \"after\" are mutually exclusive")
        }
    }
}

/// Whether to add to a `Plugin` or a `PluginGroup`.
#[derive(Debug, Clone)]
pub(crate) enum ButlerTarget {
    Plugin(TypePath),
    PluginGroup(TypePath),
}

impl ToString for ButlerTarget {
    fn to_string(&self) -> String {
        match self {
            Self::Plugin(p) => format!("Plugin({p:?})"),
            Self::PluginGroup(g) => format!("PluginGroup({g:?})"),
        }
    }
}

impl ButlerTarget {
    pub fn get_error(target1: &Self, target2: &Self) -> Error {
        match (target1, target2) {
            (ButlerTarget::Plugin(_), ButlerTarget::Plugin(_)) => Error::new(Span::call_site(), "Multiple delcarations of \"to_plugin\""),
            (ButlerTarget::PluginGroup(_), ButlerTarget::PluginGroup(_)) => Error::new(Span::call_site(), "Multiple declarations of \"to_group\""),
            _ => Error::new(Span::call_site(), "\"to_plugin\" and \"to_group\" are mutually exclusive")
        }
    }
}

pub(crate) struct AddPluginAttr {
    /// `to_group = <PluginGroup>` or `to_plugin = <Plugin>`
    pub target: Option<ButlerTarget>,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub order: Option<GroupOrdering>,
    pub init: Option<Expr>,
}

impl AddPluginAttr {
    pub fn insert_generics(&mut self, mut generics: AngleBracketedGenericArguments) -> syn::Result<()> {
        if self.generics.is_some() {
            return Err(Error::new_spanned(generics, "Multiple declarations of \"generics\""))
        }

        generics.colon2_token = Some(Default::default());
        self.generics = Some(generics);

        Ok(())
    }

    pub fn insert_target(&mut self, target: ButlerTarget) -> syn::Result<()> {
        if let Some(cur_target) = &self.target {
            return Err(ButlerTarget::get_error(cur_target, &target));
        }

        self.target = Some(target);

        Ok(())
    }

    pub fn insert_init(&mut self, init: Expr) -> syn::Result<()> {
        if self.init.is_some() {
            return Err(Error::new_spanned(init, "Multiple declarations of \"init\""));
        }

        self.init = Some(init);
        Ok(())
    }

    pub fn insert_order(&mut self, new_order: GroupOrdering) -> syn::Result<()> {
        if let Some(order) = &self.order {
            return Err(GroupOrdering::get_error(order, &new_order));
        }

        self.order = Some(new_order);
        Ok(())
    }

    pub fn require_target(&self) -> syn::Result<ButlerTarget> {
        match &self.target {
            Some(target) => Ok(target.clone()),
            None => Err(Error::new(Span::call_site(), "Expected a \"to_plugin\" or \"to_group\"")),
        }
    }

    /// Returns the statement that will be used to register this Plugin to the target
    pub fn register_statement(&self, plugin: &Ident) -> syn::Result<ExprClosure> {
        let target = self.require_target()?;
        let generics = &self.generics;
        let generics_without_colons = generics.clone().map(|mut g| { g.colon2_token = None; return g;});
        let init = self.init.as_ref().cloned().unwrap_or_else(|| parse_quote! { <#plugin #generics_without_colons as core::default::Default>::default() });
        let order = &self.order;

        Ok(match target {
            ButlerTarget::Plugin(_) => parse_quote! { |app| {
                let plugin: #plugin #generics_without_colons = {#init}.into();
                app.add_plugins(plugin);
            } },
            ButlerTarget::PluginGroup(_) => match order {
                Some(GroupOrdering::Before(other_plugin)) => parse_quote! { |builder: ::bevy_butler::__internal::bevy_app::PluginGroupBuilder| -> ::bevy_butler::__internal::bevy_app::PluginGroupBuilder {
                    let plugin: #plugin #generics_without_colons = {#init}.into();
                    builder.add_before::<#other_plugin>(plugin)
                }},
                Some(GroupOrdering::After(other_plugin)) => parse_quote! { |builder: ::bevy_butler::__internal::bevy_app::PluginGroupBuilder| -> ::bevy_butler::__internal::bevy_app::PluginGroupBuilder {
                    let plugin: #plugin #generics_without_colons = {#init}.into();
                    builder.add_after::<#other_plugin>(plugin)
                }},
                None => parse_quote! { |builder: ::bevy_butler::__internal::bevy_app::PluginGroupBuilder| -> ::bevy_butler::__internal::bevy_app::PluginGroupBuilder {
                    let plugin: #plugin #generics_without_colons = {#init}.into();
                    builder.add(plugin)
                }},
            },
        })
    }
}

impl Parse for AddPluginAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ret = AddPluginAttr {
            target: None,
            generics: None,
            init: None,
            order: None,
        };

        for arg in input.parse_terminated(GenericOrMeta::parse, Token![,])? {
            match arg {
                GenericOrMeta::Meta(meta) => {
                    match meta.path().require_ident()? {
                        ident if ident == "to_plugin" => {
                            ret.insert_target(ButlerTarget::Plugin(parse_meta_args(meta)?))?;
                        }
                        ident if ident == "to_group" => {
                            ret.insert_target(ButlerTarget::PluginGroup(parse_meta_args(meta)?))?;
                        }
                        ident if ident == "before" => {
                            ret.insert_order(GroupOrdering::Before(parse_meta_args(meta)?))?;
                        }
                        ident if ident == "after" => {
                            ret.insert_order(GroupOrdering::After(parse_meta_args(meta)?))?;
                        }
                        ident if ident == "init" => {
                            ret.insert_init(parse_meta_args(meta)?)?;
                        }
                        ident => {
                            return Err(Error::new_spanned(ident, format!("Unknown argument \"{ident}\"")));
                        }
                    }
                },
                GenericOrMeta::Generic(generics) => {
                    ret.insert_generics(generics)?;
                }
            }
        }

        Ok(ret)
    }
}