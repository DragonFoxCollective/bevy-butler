use proc_macro2::Span;
use syn::{parse::{Parse, ParseStream}, AngleBracketedGenericArguments, Error, Token, TypePath};

use crate::utils::{parse_meta_args, GenericOrMeta};

pub(crate) struct AddPluginAttr {
    pub plugin: Option<TypePath>,
    pub generics: Option<AngleBracketedGenericArguments>,
}

impl AddPluginAttr {
    pub fn insert_plugin(&mut self, plugin: TypePath) -> syn::Result<()> {
        if self.plugin.is_some() {
            return Err(Error::new_spanned(
                plugin,
                "Multiple declarations of \"plugin\"",
            ));
        }

        self.plugin = Some(plugin);
        Ok(())
    }

    pub fn insert_generics(
        &mut self,
        mut generics: AngleBracketedGenericArguments,
    ) -> syn::Result<()> {
        if self.generics.is_some() {
            return Err(Error::new_spanned(
                generics,
                "Multiple declarations of \"generics\"",
            ));
        }

        generics.colon2_token = Some(Default::default());

        self.generics = Some(generics);
        Ok(())
    }

    pub fn require_plugin(&self) -> syn::Result<&TypePath> {
        self.plugin.as_ref().ok_or(Error::new(
            Span::call_site(),
            "Expected a defined or inherited `plugin` argument",
        ))
    }
}

impl Parse for AddPluginAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ret = AddPluginAttr {
            plugin: None,
            generics: None,
        };

        for generic_or_meta in input.parse_terminated(GenericOrMeta::parse, Token![,])? {
            match generic_or_meta {
                GenericOrMeta::Generic(generics) => ret.insert_generics(generics)?,
                GenericOrMeta::Meta(meta) => match meta.path().require_ident()? {
                    ident if ident == "plugin" => {
                        ret.insert_plugin(parse_meta_args::<TypePath>(meta)?)?
                    }
                    ident => {
                        return Err(Error::new_spanned(
                            ident,
                            format!("Unknown argument \"{ident}\"")
                        ))
                    }
                }
            }
        }

        Ok(ret)
    }
}