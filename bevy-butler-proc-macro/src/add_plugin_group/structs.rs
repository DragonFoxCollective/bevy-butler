use proc_macro2::Span;
use syn::{parse::Parse, AngleBracketedGenericArguments, Error, Expr, Token};

use crate::{add_plugin::structs::ButlerTarget, utils::{parse_meta_args, GenericOrMeta}};

pub struct AddPluginGroupAttr {
    pub target: Option<ButlerTarget>,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub init: Option<Expr>,
}

impl AddPluginGroupAttr {
    pub fn insert_target(&mut self, target: ButlerTarget) -> syn::Result<()> {
        if let Some(cur_target) = &self.target {
            return Err(ButlerTarget::get_error(cur_target, &target));
        }

        self.target = Some(target);
        Ok(())
    }

    pub fn require_target(&self) -> syn::Result<&ButlerTarget> {
        self.target.as_ref().ok_or(Error::new(Span::call_site(), "Expected a \"to_plugin\" or \"to_group\""))
    }

    pub fn insert_generics(&mut self, generics: AngleBracketedGenericArguments) -> syn::Result<()> {
        if self.generics.is_some() {
            return Err(Error::new_spanned(generics, "Multiple declarations of \"generics\""));
        }

        self.generics = Some(generics);
        Ok(())
    }

    pub fn insert_init(&mut self, init: Expr) -> syn::Result<()> {
        if self.init.is_some() {
            return Err(Error::new_spanned(init, "Multiple declarations of \"init\""));
        }

        self.init = Some(init);
        Ok(())
    }
}

impl Parse for AddPluginGroupAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut ret = AddPluginGroupAttr {
            target: None,
            generics: None,
            init: None,
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
                        ident if ident == "init" => {
                            ret.insert_init(parse_meta_args(meta)?)?;
                        }
                        ident => {
                            return Err(Error::new_spanned(ident, format!("Unknown argument \"{ident}\"")))
                        }
                    }
                }
                GenericOrMeta::Generic(generic) => ret.insert_generics(generic)?,
            }
        }

        Ok(ret)
    }
}
