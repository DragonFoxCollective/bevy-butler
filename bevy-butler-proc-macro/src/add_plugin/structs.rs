use proc_macro2::Span;
use syn::{parse::{Parse, ParseStream}, parse_quote, AngleBracketedGenericArguments, Error, Expr, ExprClosure, Token, TypePath};

use crate::utils::{parse_meta_args, GenericOrMeta};

/// When adding to a `PluginGroup`, this will either
/// add using `PluginGroupBuilder.add_before` or
/// `PluginGroupBuilder.add_after`.
#[derive(Debug, Clone)]
pub(crate) enum GroupOrdering {
    Before(TypePath),
    After(TypePath),
}

/// Whether to add to a `Plugin` or a `PluginGroup`.
#[derive(Debug, Clone)]
pub(crate) enum ButlerTarget {
    Plugin(TypePath),
    PluginGroup {
        group: TypePath,
        // `before = <Plugin>` or `after = <Plugin>`
        ordering: Option<GroupOrdering>,
    },
}

pub(crate) struct AddPluginAttr {
    /// `group = <PluginGroup>` or `plugin = <Plugin>`
    pub target: Option<ButlerTarget>,
    pub generics: Option<AngleBracketedGenericArguments>,
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
            return match (cur_target, target) {
                (ButlerTarget::Plugin(_), ButlerTarget::Plugin(_)) => Err(Error::new(Span::call_site(), "Multiple delcarations of \"plugin\"")),
                (ButlerTarget::PluginGroup { .. }, ButlerTarget::PluginGroup { .. }) => Err(Error::new(Span::call_site(), "Multiple declarations of \"group\"")),
                _ => Err(Error::new(Span::call_site(), "\"add_plugin\" can either use \"group\" or \"plugin\", not both. If you want to add this plugin to both a plugin and a group, use a second \"add_plugin\" invocation."))
            }
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

    pub fn require_target(&self) -> syn::Result<ButlerTarget> {
        match &self.target {
            Some(target) => Ok(target.clone()),
            None => Err(Error::new(Span::call_site(), "Expected a \"plugin\" or \"group\"")),
        }
    }

    /// Returns the statement that will be used to register this Plugin to the target
    pub fn register_statement(&self, plugin: &TypePath) -> syn::Result<ExprClosure> {
        let target = self.require_target()?;
        let init = self.init.as_ref().cloned().unwrap_or_else(|| parse_quote! { <#plugin as core::default::Default>::default() });
        let generics = &self.generics;
        let generics_without_colons = generics.clone().map(|mut g| { g.colon2_token = None; return g;});

        Ok(match target {
            ButlerTarget::Plugin(_) => parse_quote! { |app| {
                let plugin: #plugin #generics_without_colons = {#init}.into();
                app.add_plugins(plugin);
            } },
            ButlerTarget::PluginGroup { ordering, .. } => match ordering {
                Some(GroupOrdering::Before(other_plugin)) => parse_quote! { |builder: PluginGroupBuilder| -> PluginGroupBuilder {
                    let plugin: #plugin #generics_without_colons = {#init}.into();
                    builder.add_before::<#other_plugin>(plugin);
                }},
                Some(GroupOrdering::After(other_plugin)) => parse_quote! { |builder: PluginGroupBuilder| -> PluginGroupBuilder {
                    let plugin: #plugin #generics_without_colons = {#init}.into();
                    builder.add_after::<#other_plugin>(plugin);
                }},
                None => parse_quote! { |builder: PluginGroupBuilder| -> PluginGroupBuilder {
                    let plugin: #plugin #generics_without_colons = {#init}.into();
                    builder.add(plugin);
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
        };

        // Holding variable for `before`/`after`, in case
        // it's declared before `group`
        let mut ordering: Option<GroupOrdering> = None;

        let mut insert_ordering = |new_order: GroupOrdering| -> syn::Result<()> {
            if let Some(orig_order) = &mut ordering {
                return Err(
                    match (orig_order, new_order) {
                        (GroupOrdering::Before(_), GroupOrdering::Before(_)) => Error::new(Span::call_site(), "Multiple declarations of \"before\""),
                        (GroupOrdering::After(_), GroupOrdering::After(_)) => Error::new(Span::call_site(), "Multiple declarations of \"after\""),
                        _ => Error::new(Span::call_site(), "Can only declare \"before\" or \"after\", not both")
                    }
                );
            }

            ordering = Some(new_order);
            Ok(())
        };

        for arg in input.parse_terminated(GenericOrMeta::parse, Token![,])? {
            match arg {
                GenericOrMeta::Meta(meta) => {
                    match meta.path().require_ident()? {
                        ident if ident == "plugin" => {
                            ret.insert_target(ButlerTarget::Plugin(parse_meta_args(meta)?))?;
                        }
                        ident if ident == "group" => {
                            ret.insert_target(ButlerTarget::PluginGroup { group: parse_meta_args(meta)?, ordering: None })?;
                        }
                        ident if ident == "before" => {
                            insert_ordering(GroupOrdering::Before(parse_meta_args(meta)?))?;
                        }
                        ident if ident == "after" => {
                            insert_ordering(GroupOrdering::After(parse_meta_args(meta)?))?;
                        }
                        ident if ident == "init" => {
                            ret.insert_init(parse_meta_args(meta)?)?;
                        }
                        ident => {
                            return Err(Error::new_spanned(ident, format!("Unknown argument \"{}\"", ident)));
                        }
                    }
                },
                GenericOrMeta::Generic(generics) => {
                    ret.insert_generics(generics)?;
                }
            }
        }

        // Insert ordering into the target
        if let Some(new_order) = ordering {
            ret.require_target()?;
            match ret.target.as_mut().unwrap() {
                ButlerTarget::Plugin(_) => return Err(Error::new(Span::call_site(), "\"before\" and \"after\" are only relevant for \"group\", not \"plugin\"")),
                ButlerTarget::PluginGroup { ordering, .. } => *ordering = Some(new_order),
            }
        }

        Ok(ret)
    }
}