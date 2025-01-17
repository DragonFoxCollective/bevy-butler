use quote::{quote, ToTokens};
use syn::{parse::{discouraged::Speculative, Parse, ParseStream, Parser}, punctuated::Punctuated, AngleBracketedGenericArguments, Error, ExprCall, GenericArgument, Ident, ItemFn, Meta, MetaList, MetaNameValue, Token, TypePath};

pub(crate) struct SystemAttr {
    pub plugin: Option<TypePath>,
    pub schedule: Option<TypePath>,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub transforms: Punctuated<ExprCall, Token![.]>,
}

impl SystemAttr {
    fn parse_type_path_meta(meta: Meta) -> syn::Result<TypePath> {
        match meta {
            Meta::List(list) => Ok(syn::parse2(list.tokens)?),
            Meta::NameValue(name_value) => Ok(syn::parse2(name_value.value.to_token_stream())?),
            Meta::Path(p) => Err(Error::new_spanned(p, "Expected name-value pair or list containing a TypePath")),
        }
    }

    pub fn insert_generics(&mut self, generics: AngleBracketedGenericArguments) -> syn::Result<&mut AngleBracketedGenericArguments> {
        if self.generics.is_some() {
            return Err(Error::new_spanned(generics, "Multiple declarations of \"generics\""));
        }

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
}

impl Parse for SystemAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ret = SystemAttr {
            plugin: None,
            schedule: None,
            generics: None,
            transforms: Default::default(),
        };
        // We are in a list (a = ..., b(c), ...)

        // Do some speculative parsing for `generics = <...>` because
        // syn doesn't like angle brackets in Meta
        while !input.is_empty() {
            // Fork and try to parse a Meta first
            let fork = input.fork();
            match fork.parse::<Meta>() {
                Ok(meta) => {
                    input.advance_to(&fork);
                    ret.parse_meta(meta)?;
                }
                Err(e) => {
                    // Try to parse `generics = <TypePath>`, otherwise just return the error
                    if input.parse::<Ident>().map_err(|_| e.clone())? != "generics" {
                        return Err(e);
                    }
                    input.parse::<Token![=]>().map_err(|_| e.clone())?;
                    ret.insert_generics(AngleBracketedGenericArguments::parse(input)?)?;
                }
            }
            if input.peek(Token![,])
                { input.parse::<Token![,]>()?; }
        }

        Ok(ret)
    }
}

pub(crate) struct SystemInput {
    pub attr: SystemAttr,
    pub body: ItemFn,
}

impl SystemInput {
    pub fn parse_with_attr(attr: SystemAttr) -> impl Parser<Output = Self> {
        |input: ParseStream| Ok(Self {
            attr,
            body: input.parse()?,
        })
    }
}