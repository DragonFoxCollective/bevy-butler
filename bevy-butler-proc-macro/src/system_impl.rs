//! This file enables #[system] to be used as follows
//!
//! - When attached to a free-standing function, will be registered
//! to a butler plugin as defined by its attribute args
//! - When attached to a static struct function, will be registered
//! to that struct

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream}, punctuated::Punctuated, spanned::Spanned, Attribute, Error, Expr, ExprPath, ItemFn, Meta, Path, Token
};

use crate::utils::get_crate;

#[derive(Debug, Clone)]
pub(crate) struct SystemArgs {
    pub schedule: Option<ExprPath>,
    pub plugin: Option<ExprPath>,
    pub transforms: Vec<(Path, Expr)>,
    pub span: Span,
}

impl Parse for SystemArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Self {
            schedule: None,
            plugin: None,
            transforms: Default::default(),
            span: input.span(),
        };

        loop {
            if input.is_empty() {
                break;
            }

            let meta = input.parse::<Meta>()?;
            let name_value = meta.require_name_value()?;
            match name_value
                .path
                .get_ident()
                .ok_or(input.error("Expected a name-value identifier"))?
                .to_string()
                .as_str()
            {
                "schedule" => {
                    if args.schedule.is_some() {
                        return Err(input.error("\"schedule\" defined more than once"));
                    } else if let Expr::Path(path) = name_value.value.clone() {
                        args.schedule = Some(path);
                    } else {
                        return Err(input.error("Expected a Schedule after \"schedule\""));
                    }
                }
                "plugin" => {
                    if args.plugin.is_some() {
                        return Err(input.error("\"plugin\" defined more than once"));
                    } else if let Expr::Path(path) = name_value.value.clone() {
                        args.plugin = Some(path);
                    } else {
                        return Err(input.error("Expected a Plugin after \"plugin\""));
                    }
                }
                _ => {
                    // Any other attributes, assume they're transformers for the system
                    args.transforms
                        .push((name_value.path.clone(), name_value.value.clone()));
                }
            }

            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
        }

        Ok(args)
    }
}

impl SystemArgs {
    /// Returns a new SystemArgs, using `self` as the default values
    /// and `new_args` as the overriding arguments.
    pub fn splat(&self, new_args: &SystemArgs) -> SystemArgs {
        Self {
            plugin: new_args.plugin.clone().or(self.plugin.clone()),
            schedule: new_args.schedule.clone().or(self.schedule.clone()),
            transforms: [self.transforms.clone(), new_args.transforms.clone()].concat(),
            span: new_args.span,
        }
    }
}

impl ToTokens for SystemArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(value) = self.plugin.clone() {
            tokens.extend(quote! { plugin = #value, });
        }
        if let Some(value) = self.schedule.clone() {
            tokens.extend(quote! { schedule = #value, });
        }
        for (path, value) in &self.transforms {
            tokens.extend(quote! { #path = #value, });
        }
    }
}

pub(crate) struct SystemAttr {
    pub span: Span,
    pub args: Option<SystemArgs>,
}

impl TryFrom<&Attribute> for SystemAttr {
    type Error = syn::Error;

    fn try_from(value: &Attribute) -> Result<Self, Self::Error> {
        if value.path().get_ident().is_none_or(|ident| ident != "system") {
            return Err(Error::new(value.path().span(), "Expected \"system\""));
        }

        match value.meta {
            Meta::Path(_) => Ok(Self { span: value.span(), args: None }),
            Meta::NameValue(_) => Ok(Self { span: value.span(), args: Some(value.parse_args()?) }),
            Meta::List(_) => Err(Error::new(value.span(), "Unexpected list in #[system] declaration"))
        }
    }
}

impl SystemAttr {
    pub fn require_args(&self) -> Result<SystemArgs, proc_macro2::TokenStream> {
        match &self.args {
            None => Err(Error::new(self.span, "Expected name-value args").to_compile_error()),
            Some(args) => Ok(args.clone())
        }
    }
}

pub struct SystemInput {
    pub attr: SystemAttr,
    pub item: ItemFn,
}

impl Parse for SystemInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item = input.parse::<ItemFn>()?;
        let attr = {
            let mut sys_attr = None;
            for attr in &item.attrs {
                if attr.path().get_ident().is_some_and(|i| i == "system") {
                    sys_attr = Some(SystemAttr::try_from(attr)?);
                    break;
                }
            }
            sys_attr.ok_or(Error::new(item.span(), "Expected #[system]"))?
        };

        Ok(Self {
            attr,
            item,
        })
    }
}

/// Implementation for `#[system]` on free-standing functions
pub(crate) fn free_standing_impl(input: SystemInput) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
    let args = input.attr.require_args()?;
    let item = input.item;

    let schedule = args.schedule.ok_or_else(|| {
        Error::new(
            args.span,
            "#[system] requires either a defined or inherited `schedule`",
        )
        .into_compile_error()
    })?;
    let plugin = args.plugin.ok_or_else(|| {
        Error::new(
            args.span,
            "#[system] requires either a defined or inherited `plugin`",
        )
        .into_compile_error()
    })?;

    let bevy_butler = get_crate("bevy-butler")
        .map_err(|e| Error::new(Span::call_site(), e).to_compile_error())?;

    let sys_name = &item.sig.ident;

    let sys_transform = {
        let mut punctuated = Punctuated::<Expr, syn::token::Dot>::new();
        punctuated.push(syn::parse2(sys_name.into_token_stream()).unwrap());
        for (path, args) in args.transforms {
            punctuated.push(syn::parse2(quote!( #path(#args) )).unwrap());
        }
        punctuated
    };

    let butler_sys_name = format_ident!("__butler_{}", sys_name);
    let static_name = format_ident!("__static_{}", sys_name);

    Ok(quote! {
        #item

        #[allow(non_camel_case_types)]
        struct #butler_sys_name;

        impl #bevy_butler::__internal::ButlerSystem for #butler_sys_name {
            type Plugin = #plugin;

            fn system(&self) -> fn(&mut App) {
                use #bevy_butler::__internal::ButlerPlugin;
                #plugin::_marker().internal_crate_protection_marker;
                |app| { app.add_systems(#schedule, #sys_transform); }
            }
        }

        #[#bevy_butler::__internal::linkme::distributed_slice(#bevy_butler::__internal::BUTLER_SLICE)]
        #[linkme(crate = #bevy_butler::__internal::linkme)] // I LOVE UNDOCUMENTED ATTRIBUTES!!! FUCK!!!
        #[allow(non_upper_case_globals)]
        static #static_name: &'static dyn #bevy_butler::__internal::ButlerStaticSystem = & #butler_sys_name;
    }.into())
}
