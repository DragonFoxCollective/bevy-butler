//! This file enables #[system] to be used as follows
//! 
//! - When attached to a free-standing function, will be registered
//! to a butler plugin as defined by its attribute args
//! - When attached to a static struct function, will be registered
//! to that struct

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse::{Parse, ParseStream}, parse_macro_input, Error, Expr, ExprCall, ExprPath, ItemFn, Meta, Token};
use itertools::Itertools;

use crate::utils::get_crate;

struct SystemArgs {
    pub schedule: Option<ExprPath>,
    pub plugin: Option<ExprPath>,
    pub transforms: Vec<ExprCall>,
}

impl Parse for SystemArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Self {
            schedule: None,
            plugin: None,
            transforms: Default::default(),
        };

        loop {
            let meta = input.parse::<Meta>()?;
            let name_value = meta.require_name_value()?;
            match name_value.path
                .get_ident()
                .ok_or(input.error("Expected a name-value identifier"))?
                .to_string()
                .as_str()
            {
                "schedule" => {
                    if args.schedule.is_some() {
                        return Err(input.error("\"schedule\" defined more than once"));
                    }
                    else if let Expr::Path(path) = name_value.value.clone() {
                        args.schedule = Some(path);
                    }
                    else {
                        return Err(input.error("Expected a Schedule after \"schedule\""));
                    }
                },
                "plugin" => {
                    if args.plugin.is_some() {
                        return Err(input.error("\"plugin\" defined more than once"));
                    }
                    else if let Expr::Path(path) = name_value.value.clone() {
                        args.plugin = Some(path);
                    }
                    else {
                        return Err(input.error("Expected a Plugin after \"plugin\""));
                    }
                },
                ident => {
                    // Any other attributes, assume they're transformers for the system
                    let transform_str = format!("{}({})", ident, name_value.value.to_token_stream().to_string());
                    let call: ExprCall = syn::parse_str(&transform_str)?;
                    args.transforms
                        .push(call);
                }
            }

            if input.is_empty() {
                break;
            }
            else {
                input.parse::<Token![,]>()?;
                // Allow trailing commas
                if input.is_empty() {
                    break;
                }
            }
        }

        Ok(args)
    }
}

/// Implementation for `#[system]` on free-standing functions
/// 
/// ```
/// # use bevy_butler_proc_macro::*;
/// # use bevy_butler::BevyButlerPlugin;
/// # use bevy::prelude::*;
/// # #[butler_plugin]
/// # struct MyPlugin;
/// #[system(schedule = Startup, plugin = MyPlugin, run_if = || true)]
/// fn hello_world() {
///     info!("Hello, world!")
/// }
/// #
/// # fn main() {
/// #   App::new().add_plugins((BevyButlerPlugin, MyPlugin)).run();
/// # }
/// #
/// ```
pub(crate) fn system_free_standing_impl(args: TokenStream, item: ItemFn) -> TokenStream {
    let args = parse_macro_input!(args as SystemArgs);

    if args.schedule.is_none() {
        return Error::new(Span::call_site(), "#[system] requires either a defined or inherited schedule").into_compile_error().into();
    }
    let schedule = args.schedule.unwrap();

    let bevy_butler = get_crate("bevy-butler");
    if let Err(e) = bevy_butler {
        return Error::new(Span::call_site(), e).to_compile_error().into();
    }
    let bevy_butler = bevy_butler.unwrap();

    let sys_name = &item.sig.ident;
    let plugin: Expr = match args.plugin {
        Some(plugin) => Expr::Path(plugin),
        None => syn::parse2(quote!{ #bevy_butler::BevyButlerPlugin }).unwrap(),
    };

    let transform_str = args.transforms
        .into_iter()
        .map(|t| t.to_token_stream().to_string())
        .join(".");
    let transforms: Option<Expr> = if !transform_str.is_empty() {
        Some(syn::parse_str(&transform_str).unwrap())
    } else {
        None
    };
    let period = if transforms.is_some() { Some(quote!(.))} else { None };

    quote! {
        #item

        #bevy_butler::__internal::inventory::submit! {
            #bevy_butler::__internal::ButlerFunc::new::<#plugin>(|app| { app.add_systems( #schedule, #sys_name #period #transforms ); })
        } 
    }.into()
}