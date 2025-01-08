//! This file enables #[system] to be used as follows
//! 
//! - When attached to a free-standing function, will be registered
//! to a butler plugin as defined by its attribute args
//! - When attached to a static struct function, will be registered
//! to that struct

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse::{Parse, ParseStream}, parse_macro_input, Error, Expr, ExprCall, ExprPath, Ident, ItemFn, Meta, Token};

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
                    args.transforms
                        .push(syn::parse_str(&format!("{}({})", ident, name_value.value.to_token_stream().to_string()))?);
                }
            }

            if input.is_empty() {
                break;
            }
            else {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(args)
    }
}

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
    let plugin: Expr = args.plugin.unwrap_or(syn::parse_str("()").unwrap());

    let call_site = proc_macro::Span::call_site();
    let source_file = call_site.source_file();
    let line = call_site.line();

    let digest: syn::Result<Ident> = syn::parse_str(&sha256::digest(format!("{}{}{}", sys_name.to_string(), source_file.path().to_string_lossy(), line)));
    if let Err(e) = digest {
        return e.to_compile_error().into();
    }
    let digest = digest.unwrap();

    let mut transform_str = String::new();
    for transform in args.transforms.into_iter() {
        transform_str += &format!(".{}", transform.to_token_stream().to_string());
    }
    let transforms: syn::Result<Expr> = syn::parse_str(&transform_str);
    if let Err(e) = transforms {
        return e.into_compile_error().into();
    }
    let transforms = transforms.unwrap();

    let butler_func_name: syn::Result<Ident> = syn::parse_str(&format!("__butler_{}", digest));
    if let Err(e) = butler_func_name {
        return e.into_compile_error().into();
    }
    let butler_func_name = butler_func_name.unwrap();

    quote! {
        #item

        fn __butler_ #digest (app: &mut ) {
            app.add_systems(#schedule, #sys_name.#transforms);
        }

        inventory::submit! {
            #bevy_butler::__internal::ButlerFunc::new::<#plugin>(#)
        }
    }.into()
}