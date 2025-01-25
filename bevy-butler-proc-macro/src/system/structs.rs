use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{Item, ItemUse};
use syn::{parse::{Parse, ParseStream, Parser}, punctuated::Punctuated, AngleBracketedGenericArguments, Attribute, Error, ExprCall, GenericArgument, ItemFn, Meta, MetaList, MetaNameValue, Token, TypePath};

use crate::utils;

#[derive(Clone)]
pub(crate) struct SystemAttr {
    pub plugin: Option<TypePath>,
    pub schedule: Option<TypePath>,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub transforms: Punctuated<ExprCall, Token![.]>,
    pub attr_span: Span,
}

impl Default for SystemAttr {
    fn default() -> Self {
        Self {
            plugin: None,
            schedule: None,
            generics: None,
            transforms: Default::default(),
            attr_span: Span::call_site(),
        }
    }
}

impl SystemAttr {
    pub fn require_plugin(&self) -> syn::Result<&TypePath> {
        self.plugin.as_ref().ok_or(Error::new(self.attr_span, "Expected a defined or inherited `plugin` argument"))
    }

    pub fn require_schedule(&self) -> syn::Result<&TypePath> {
        self.schedule.as_ref().ok_or(Error::new(self.attr_span, "Expected a defined or inherited `schedule` argument"))
    }

    pub fn with_defaults(&mut self, defaults: Self) {
        self.generics = self.generics.take().or(defaults.generics);
        self.schedule = self.schedule.take().or(defaults.schedule);
        self.plugin = self.plugin.take().or(defaults.plugin);

        // Append our transforms onto the end of the defaults
        let mut transforms = defaults.transforms;
        transforms.extend(std::mem::take(&mut self.transforms));
        self.transforms = transforms;
    }

    fn parse_type_path_meta(meta: Meta) -> syn::Result<TypePath> {
        match meta {
            Meta::List(list) => Ok(syn::parse2(list.tokens)?),
            Meta::NameValue(name_value) => Ok(syn::parse2(name_value.value.to_token_stream())?),
            Meta::Path(p) => Err(Error::new_spanned(p, "Expected name-value pair or list containing a TypePath")),
        }
    }

    pub fn insert_generics(&mut self, mut generics: AngleBracketedGenericArguments) -> syn::Result<&mut AngleBracketedGenericArguments> {
        if self.generics.is_some() {
            return Err(Error::new_spanned(generics, "Multiple declarations of \"generics\""));
        }

        generics.colon2_token = Some(Default::default());

        Ok(self.generics.insert(generics))
    }

    pub fn parse_plugin_meta(&mut self, meta: Meta) -> syn::Result<&mut TypePath> {
        if self.plugin.is_some() {
            return Err(Error::new_spanned(meta, "Multiple declarations of \"plugin\""));
        }

        Ok(self.plugin.insert(Self::parse_type_path_meta(meta)?))
    }

    pub fn parse_schedule_meta(&mut self, meta: Meta) -> syn::Result<&mut TypePath> {
        if self.schedule.is_some() {
            return Err(Error::new_spanned(meta, "Multiple declarations of \"schedule\""));
        }

        Ok(self.schedule.insert(Self::parse_type_path_meta(meta)?))
    }

    pub fn parse_transform_meta(&mut self, meta: Meta) -> syn::Result<&mut Punctuated<ExprCall, Token![.]>> {
        let expr: ExprCall = match meta {
            // No-argument transform like `chain()`
            Meta::Path(path) => syn::parse2(quote!(#path ()))?,

            // Single argument transform like `run_if(some_condition)`
            Meta::NameValue(MetaNameValue { path, value, .. }) => syn::parse2(quote!(#path (#value)))?,

            // Multiple argument transform (currently doesn't exist within Bevy but may be a user-defined transform)
            Meta::List(MetaList { path, tokens, ..}) => syn::parse2(quote!(#path (#tokens)))?,
        };

        self.transforms.push(expr);
        Ok(&mut self.transforms)
    }

    pub fn parse_generics_meta(&mut self, meta: Meta) -> syn::Result<&mut AngleBracketedGenericArguments> {
        let mut generics = AngleBracketedGenericArguments {
            colon2_token: Some(Default::default()),
            lt_token: Default::default(),
            gt_token: Default::default(),
            args: Default::default(),
        };

        match meta {
            Meta::List(list) => generics.args = list.parse_args_with(Punctuated::<GenericArgument, Token![,]>::parse_terminated)?,
            Meta::NameValue(name_value) => generics.args = Punctuated::<GenericArgument, Token![,]>::parse_terminated.parse2(name_value.value.to_token_stream())?,
            Meta::Path(p) => return Err(Error::new_spanned(p, "Expected name-value pair or list containing generic arguments")),
        }

        Ok(self.insert_generics(generics)?)
    }

    pub fn parse_meta(&mut self, meta: Meta) -> syn::Result<()> {
        match meta.path().get_ident() {
            Some(ident) if ident == "plugin" => { self.parse_plugin_meta(meta)?; }
            Some(ident) if ident == "schedule" => { self.parse_schedule_meta(meta)?; }
            Some(ident) if ident == "generics" => { self.parse_generics_meta(meta)?; }
            Some(_) | None => { self.parse_transform_meta(meta)?; }
        }

        Ok(())
    }

    /// Tries to parse the given attribute into a #[system] attribute
    /// Returns Ok(None) if the attribute's ident is not "system"
    pub fn try_parse_system_attr(attr: &Attribute) -> syn::Result<Option<Self>> {
        if attr.path().get_ident().is_none_or(|i| i != "system") {
            return Ok(None);
        }

        if matches!(attr.meta, Meta::Path(_)) {
            return Ok(Some(SystemAttr::default()));
        }

        Ok(Some(attr.parse_args()?))
    }

    pub fn get_metas(&self) -> Punctuated<Meta, Token![,]> {
        let mut args = Punctuated::<Meta, Token![,]>::new();
        if let Some(meta) = self.plugin.as_ref().map(|plugin| syn::parse_quote!(plugin = #plugin)) {
            args.push(meta);
        }
        if let Some(meta) = self.schedule.as_ref().map(|schedule| syn::parse_quote!(schedule = #schedule)) {
            args.push(meta);
        }
        if let Some(meta) = self.generics.as_ref().map(|generics| {
            let generics = &generics.args;
            syn::parse_quote!(generics(#generics))
        }) {
            args.push(meta);
        }
        self.transforms.iter().for_each(|trns| {
            args.push(syn::parse_quote!(#trns));
        });

        args
    }
}

impl From<SystemAttr> for MetaList {
    fn from(value: SystemAttr) -> Self {
        let metas = value.get_metas();
        MetaList {
            delimiter: syn::MacroDelimiter::Paren(Default::default()),
            path: syn::parse_quote!(system),
            tokens: quote!(#metas),
        }
    }
}

impl From<SystemAttr> for Meta {
    fn from(value: SystemAttr) -> Self {
        Meta::List(value.into())
    }
}

impl Parse for SystemAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ret = SystemAttr {
            plugin: None,
            schedule: None,
            generics: None,
            transforms: Default::default(),
            attr_span: input.span(),
        };

        enum GenericOrMeta {
            Generic(AngleBracketedGenericArguments),
            Meta(Meta),
        }

        let parser = |input: ParseStream| -> syn::Result<GenericOrMeta> {
            // See if we're parsing a `generics`
            if let Some(generics) = utils::try_parse_generics_arg(input)? {
                Ok(GenericOrMeta::Generic(generics))
            }
            else {
                Ok(GenericOrMeta::Meta(input.parse()?))
            }
        };

        // We are in a list (a = ..., b(c), ...)
        for arg in input.parse_terminated(parser, Token![,])? {
            match arg {
                GenericOrMeta::Generic(g) => { ret.insert_generics(g)?; },
                GenericOrMeta::Meta(m) => { ret.parse_meta(m)?; },
            }
        }

        Ok(ret)
    }
}

pub(crate) enum SystemInput {
    Fn {
        attr: SystemAttr,
        body: ItemFn,
    },
    Use {
        attr: SystemAttr,
        body: ItemUse,
    }
}

impl SystemInput {
    pub fn parse_with_attr(attr: SystemAttr) -> impl Parser<Output = Self> {
        |input: ParseStream| {
            match input.parse::<Item>()? {
                Item::Fn(body) => Ok(Self::Fn { attr, body }),
                Item::Use(body) => Ok(Self::Use { attr, body }),
                item => Err(Error::new_spanned(item, "Expected a free-standing fn or a use declaration block"))
            }
        }
    }
}