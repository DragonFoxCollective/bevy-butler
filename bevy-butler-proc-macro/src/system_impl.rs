//! This file enables #[system] to be used as follows
//!
//! - When attached to a free-standing function, will be registered
//! to a butler plugin as defined by its attribute args
//! - When attached to a static struct function, will be registered
//! to that struct

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Error, Expr, ExprPath, Ident, ItemFn, Meta, MetaList, Token,
};

#[derive(Clone)]
pub(crate) struct SystemArgs {
    pub schedule: Option<ExprPath>,
    pub plugin: Option<ExprPath>,
    pub transforms: Vec<(ExprPath, Option<Expr>)>,
    pub span: Span,
}

pub(crate) fn register_system_set_block(
    systems: &[(Ident, Option<SystemArgs>)],
    args: &SystemArgs,
) -> TokenStream {
    let plugin = args.require_plugin().unwrap();
    let schedule = args.require_schedule().unwrap();

    let mut sys_set = Punctuated::<TokenStream, syn::token::Comma>::new();
    for (system, args) in systems {
        match args {
            None => sys_set.push(quote!(#system)),
            Some(args) => {
                sys_set.push(args.transform_system(&syn::parse2(system.to_token_stream()).unwrap()))
            }
        }
    }

    let systems_strings: Vec<u8> = systems
        .iter()
        .map(|(ident, _)| ident.to_string().into_bytes())
        .flatten()
        .collect();
    let hash = sha256::digest(systems_strings);
    let butler_sys_struct = format_ident!("_butler_{}", hash);
    let static_name = format_ident!("_butler_static_{}", hash);

    let sys_transform = args.transform_system(&sys_set.to_token_stream());

    quote! {
        #[allow(non_camel_case_types)]
        struct #butler_sys_struct;

        impl ::bevy_butler::__internal::ButlerSystem for #butler_sys_struct {
            type Plugin = #plugin;

            fn system(&self) -> fn(&mut App) {
                use ::bevy_butler::__internal::ButlerPlugin;
                #plugin::_marker().internal_crate_protection_marker;
                |app| { app.add_systems(#schedule, #sys_transform); }
            }
        }

        #[::bevy_butler::__internal::linkme::distributed_slice(::bevy_butler::__internal::BUTLER_SLICE)]
        #[linkme(crate = ::bevy_butler::__internal::linkme)] // I LOVE UNDOCUMENTED ATTRIBUTES!!! FUCK!!!
        #[allow(non_upper_case_globals)]
        static #static_name: &'static dyn ::bevy_butler::__internal::ButlerStaticSystem = & #butler_sys_struct;
    }
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

            match meta {
                Meta::Path(path) => {
                    // Just a function to call on the system, no args
                    let ident = path.get_ident();
                    if let Some(ident) = ident {
                        if ident == "schedule" || ident == "plugin" {
                            return Err(Error::new(
                                ident.span(),
                                &format!("Expected `{ident}` to be a name-value attribute"),
                            ));
                        }
                    }

                    args.transforms.push((
                        ExprPath {
                            attrs: vec![],
                            qself: None,
                            path,
                        },
                        None,
                    ));
                }
                Meta::List(list) => {
                    // A proper function call
                    let ident = list.path.get_ident();
                    if let Some(ident) = ident {
                        if ident == "schedule" || ident == "plugin" {
                            return Err(Error::new(
                                ident.span(),
                                &format!("Expected `{ident}` to be a name-value attribute"),
                            ));
                        }
                    }

                    let exprs = list.tokens;

                    args.transforms.push((
                        ExprPath {
                            attrs: vec![],
                            qself: None,
                            path: list.path,
                        },
                        Some(syn::Expr::Verbatim(exprs)),
                    ));
                }
                Meta::NameValue(name_value) => {
                    // An attribute like `run_if = || true`
                    match name_value.path.get_ident() {
                        Some(ident) if ident == "schedule" => {
                            if args.schedule.is_some() {
                                return Err(Error::new(
                                    ident.span(),
                                    "`schedule` defined more than once",
                                ));
                            } else {
                                args.schedule =
                                    Some(syn::parse2(name_value.value.to_token_stream())?);
                            }
                        }
                        Some(ident) if ident == "plugin" => {
                            if args.plugin.is_some() {
                                return Err(Error::new(
                                    ident.span(),
                                    "`schedule` defined more than once",
                                ));
                            } else {
                                args.plugin =
                                    Some(syn::parse2(name_value.value.to_token_stream())?);
                            }
                        }
                        Some(_) | None => {
                            args.transforms.push((
                                ExprPath {
                                    attrs: vec![],
                                    qself: None,
                                    path: name_value.path,
                                },
                                Some(name_value.value),
                            ));
                        }
                    }
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

    pub fn transform_system(&self, system: &TokenStream) -> TokenStream {
        let mut punct = Punctuated::<TokenStream, syn::token::Dot>::new();
        punct.push(quote! { (#system) });

        self.transforms
            .iter()
            .for_each(|(ident, args)| punct.push(quote! { #ident(#args) }));

        punct.to_token_stream()
    }

    pub fn require_plugin(&self) -> Result<&ExprPath, TokenStream> {
        match &self.plugin {
            None => {
                Err(Error::new(self.span, "Expected a `plugin` name-value").into_compile_error())
            }
            Some(plugin) => Ok(plugin),
        }
    }

    pub fn require_schedule(&self) -> Result<&ExprPath, TokenStream> {
        match &self.schedule {
            None => {
                Err(Error::new(self.span, "Expected a `plugin` name-value").into_compile_error())
            }
            Some(schedule) => Ok(schedule),
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
        if value
            .path()
            .get_ident()
            .is_none_or(|ident| ident != "system")
        {
            return Err(Error::new(value.path().span(), "Expected \"system\""));
        }

        match value.meta {
            Meta::Path(_) => Ok(Self {
                span: value.span(),
                args: None,
            }),
            _ => Ok(Self {
                span: value.span(),
                args: Some(value.parse_args()?),
            }),
        }
    }
}

impl Into<Attribute> for &SystemAttr {
    fn into(self) -> Attribute {
        let meta = match &self.args {
            None => Meta::Path(syn::parse_str("system").unwrap()),
            Some(args) => Meta::List(MetaList {
                path: syn::parse_str("system").unwrap(),
                delimiter: syn::MacroDelimiter::Brace(Default::default()),
                tokens: args.into_token_stream(),
            }),
        };

        Attribute {
            pound_token: Default::default(),
            style: syn::AttrStyle::Outer,
            bracket_token: Default::default(),
            meta,
        }
    }
}

impl SystemAttr {
    pub fn require_args(&self) -> Result<SystemArgs, proc_macro2::TokenStream> {
        match &self.args {
            None => Err(Error::new(self.span, "Expected name-value args").to_compile_error()),
            Some(args) => Ok(args.clone()),
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

        Ok(Self { attr, item })
    }
}

/// Implementation for `#[system]` on free-standing functions
pub(crate) fn free_standing_impl(
    input: SystemInput,
) -> Result<proc_macro2::TokenStream, proc_macro2::TokenStream> {
    let args = input.attr.require_args()?;
    let item = input.item;

    args.require_plugin()?;
    args.require_schedule()?;

    let sys_name = &item.sig.ident;

    let register_block = register_system_set_block(&vec![(sys_name.clone(), None)], &args);

    Ok(quote! {
        #item

        #register_block
    }
    .into())
}
