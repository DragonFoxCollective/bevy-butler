use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::spanned::Spanned;
use syn::{parse::{Parse, ParseStream, Parser}, punctuated::Punctuated, Error, Expr, ItemFn, Meta, MetaList, Token, TypePath};

#[derive(Clone)]
pub(crate) struct CustomSystemAttr {
    pub plugin: Option<TypePath>,
    pub schedule: Option<TypePath>,
    pub transforms: Option<Expr>,
    pub attr_span: Span,
}

impl Default for CustomSystemAttr {
    fn default() -> Self {
        Self {
            plugin: None,
            schedule: None,
            transforms: Default::default(),
            attr_span: Span::call_site(),
        }
    }
}

impl CustomSystemAttr {
    pub fn require_plugin(&self) -> syn::Result<&TypePath> {
        self.plugin.as_ref().ok_or(Error::new(self.attr_span, "Expected a defined or inherited `plugin` argument"))
    }

    pub fn require_schedule(&self) -> syn::Result<&TypePath> {
        self.schedule.as_ref().ok_or(Error::new(self.attr_span, "Expected a defined or inherited `schedule` argument"))
    }

	pub fn require_transforms(&self) -> syn::Result<&Expr> {
        self.transforms.as_ref().ok_or(Error::new(self.attr_span, "Expected a defined or inherited `transforms` argument"))
    }

    fn parse_type_path_meta(meta: Meta) -> syn::Result<TypePath> {
        match meta {
            Meta::List(list) => Ok(syn::parse2(list.tokens)?),
            Meta::NameValue(name_value) => Ok(syn::parse2(name_value.value.to_token_stream())?),
            Meta::Path(p) => Err(Error::new_spanned(p, "Expected name-value pair or list containing a TypePath")),
        }
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

    pub fn parse_transforms_meta(&mut self, meta: Meta) -> syn::Result<&mut Expr> {
        if self.transforms.is_some() {
            return Err(Error::new_spanned(meta, "Multiple declarations of \"transforms\""));
        }

        let expr = match meta {
            Meta::List(list) => Ok(syn::parse2(list.tokens)?),
            Meta::NameValue(name_value) => Ok(syn::parse2(name_value.value.to_token_stream())?),
            Meta::Path(p) => Err(Error::new_spanned(p, "Expected name-value pair or list containing a TypePath")),
        };

        Ok(self.transforms.insert(expr?))
    }

    pub fn parse_meta(&mut self, meta: Meta) -> syn::Result<()> {
        match meta.path().get_ident() {
            Some(ident) if ident == "plugin" => { self.parse_plugin_meta(meta)?; }
            Some(ident) if ident == "schedule" => { self.parse_schedule_meta(meta)?; }
            Some(ident) if ident == "transforms" => { self.parse_transforms_meta(meta)?; }
            Some(_) | None => { return Err(Error::new(meta.path().span(), "Expected `generics`, `plugin`, or `transforms`")); }
        }

        Ok(())
    }

    pub fn get_metas(&self) -> Punctuated<Meta, Token![,]> {
        let mut args = Punctuated::<Meta, Token![,]>::new();
        if let Some(meta) = self.plugin.as_ref().map(|plugin| syn::parse_quote!(plugin = #plugin)) {
            args.push(meta);
        }
        if let Some(meta) = self.schedule.as_ref().map(|schedule| syn::parse_quote!(schedule = #schedule)) {
            args.push(meta);
        }
        if let Some(meta) = self.transforms.as_ref().map(|transforms| syn::parse_quote!(transforms = #transforms)) {
            args.push(meta);
        }

        args
    }
}

impl From<CustomSystemAttr> for MetaList {
    fn from(value: CustomSystemAttr) -> Self {
        let metas = value.get_metas();
        MetaList {
            delimiter: syn::MacroDelimiter::Paren(Default::default()),
            path: syn::parse_quote!(system),
            tokens: quote!(#metas),
        }
    }
}

impl From<CustomSystemAttr> for Meta {
    fn from(value: CustomSystemAttr) -> Self {
        Meta::List(value.into())
    }
}

impl Parse for CustomSystemAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ret = CustomSystemAttr {
            plugin: None,
            schedule: None,
            transforms: Default::default(),
            attr_span: input.span(),
        };

        // We are in a list (a = ..., b(c), ...)
        for arg in Punctuated::<Meta, Token![,]>::parse_terminated(input)? {
            ret.parse_meta(arg)?;
        }

        Ok(ret)
    }
}

pub(crate) struct SystemInput {
    pub attr: CustomSystemAttr,
    pub body: ItemFn,
}

impl SystemInput {
    pub fn parse_with_attr(attr: CustomSystemAttr) -> impl Parser<Output = Self> {
        |input: ParseStream| {
            let body: ItemFn = input.parse()?;
            Ok(Self {
                attr,
                body,
            })
        }
    }
}