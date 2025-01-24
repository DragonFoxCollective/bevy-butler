use proc_macro2::Span;
use quote::ToTokens;
use syn::{parse::{Parse, ParseStream}, AngleBracketedGenericArguments, Error, Ident, ItemFn, Meta, Token, TypePath};

use crate::utils;

pub(crate) struct ObserverAttr {
    pub plugin: Option<TypePath>,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub attr_span: Span,
}

impl ObserverAttr {
    pub fn require_plugin(&self) -> syn::Result<&TypePath> {
        self.plugin.as_ref().ok_or(Error::new(self.attr_span, "Expected a defined or inherited `plugin` argument"))
    }

    pub fn insert_plugin(&mut self, plugin: TypePath) -> syn::Result<&mut TypePath> {
        if self.plugin.is_some() {
            return Err(Error::new_spanned(plugin, "Multiple declarations of \"plugin\""));
        }

        Ok(self.plugin.insert(plugin))
    }

    pub fn insert_generics(&mut self, mut generics: AngleBracketedGenericArguments) -> syn::Result<&mut AngleBracketedGenericArguments> {
        if self.generics.is_some() {
            return Err(Error::new_spanned(generics, "Multiple declarations of \"generics\""));
        }

        generics.colon2_token = Some(Default::default());

        Ok(self.generics.insert(generics))
    }
}

impl Parse for ObserverAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ret = Self {
            attr_span: Span::call_site(),
            plugin: None,
            generics: None,
        };

        while !input.is_empty() {
            // Try to parse `generics`
            if let Some(generics) = utils::try_parse_generics_arg(input)? {
                ret.insert_generics(generics)?;
            }
            else {
                let ident = input.fork().parse::<Ident>()?;
                if ident != "plugin" {
                    return Err(Error::new_spanned(ident, "Expected `generics` or `plugin`"));
                }
                // Try to parse `plugin`
                let plugin = match input.parse::<Meta>()? {
                    Meta::List(l) => l.parse_args(),
                    Meta::NameValue(name_value) => syn::parse2(name_value.value.into_token_stream()),
                    Meta::Path(p) => Err(Error::new_spanned(p, "Expected `plugin = ...` or `plugin(...)`")),
                };

                ret.insert_plugin(plugin?)?;
            }
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(ret)
    }
}

pub(crate) struct ObserverInput {
    pub attr: ObserverAttr,
    pub func: ItemFn,
}