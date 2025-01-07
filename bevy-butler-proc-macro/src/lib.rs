use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse::{Parse, ParseStream}, parse_macro_input, Expr, ExprPath, ItemFn, ItemStruct, Meta, Path, Token};

struct SystemArgs {
    schedule: ExprPath,
    plugin: Option<ExprPath>,
    transforms: Option<Expr>,
}

impl Parse for SystemArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut schedule: Option<ExprPath> = None;
        let mut plugin: Option<ExprPath> = None;
        let mut transforms: Option<Expr> = None;

        loop {
            let meta = input.parse::<Meta>()?;
            let name_value = meta.require_name_value()?;
            match name_value.path.get_ident().ok_or(input.error("Expected a name-value identifier"))? {
                ident if ident == "schedule" => {
                    match &name_value.value {
                        Expr::Path(path) => schedule = Some(path.clone()),
                        _ => return Err(input.error("Expected a Schedule")),
                    }
                },
                ident if ident == "plugin" => {
                    match &name_value.value {
                        Expr::Path(path) => plugin = Some(path.clone()),
                        _ => return Err(input.error("Expected a Plugin")),
                    }
                },
                ident if ident == "transforms" => {
                    transforms = Some(name_value.value.clone());
                }
                _ => {
                    return Err(input.error(format!("Unknown attribute \"{:?}\"", name_value.path)));
                }
            }

            if input.is_empty() {
                break;
            }
            else {
                input.parse::<Token![,]>()?;
            }
        }

        let schedule = schedule.ok_or(input.error("#[system] requires a \"schedule\""))?;
        Ok(Self {
            schedule,
            plugin,
            transforms
        })
    }
}

#[proc_macro_attribute]
pub fn system(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let args = parse_macro_input!(attr as SystemArgs);
    let schedule = args.schedule;
    let default_plugin = syn::parse_str("::bevy_butler::BevyButlerPlugin").unwrap();
    let plugin = args.plugin.unwrap_or(default_plugin);
    let func_name = input.sig.ident.clone();
    let transformed_func = args.transforms
        .map(|transforms| quote! { #func_name.#transforms})
        .unwrap_or_else(|| func_name.clone().into_token_stream());

    let butler_func_name = format_ident!("_butler_{}", func_name);

    quote! {
        #input

        fn #butler_func_name (plugin: &#plugin, app: &mut bevy::prelude::App) {
            app.add_systems(#schedule, #transformed_func);
        }

        ::bevy_butler::__internal::inventory::submit! {
            ::bevy_butler::__internal::ButlerFunc::new::<#plugin>(#butler_func_name)
        }
    }.into()
}

#[proc_macro_attribute]
pub fn auto_plugin(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let plugin = &input.ident;

    quote! {
        #input

        impl ::bevy::prelude::Plugin for #plugin {
            fn build(&self, app: &mut ::bevy::app::App) {
                let funcs = app.world().get_resource_ref::<::bevy_butler::__internal::ButlerRegistry>()
                    .unwrap_or_else(|| panic!("Tried to build an #[auto_plugin] without adding BevyButlerPlugin first!"))
                    .get_funcs::<#plugin>();

                let mut sys_count = 0;
                if let Some(funcs) = funcs {
                    for butler_func in &(*funcs) {
                        if let Some(sys) = butler_func.try_get_func::<#plugin>() {
                            (sys)(self, app);
                            sys_count += 1;
                        }
                    }
                }
                
                ::bevy_butler::__internal::_butler_debug(&format!("{} added {sys_count} systems", stringify!(#plugin)));
            }
        }
    }.into()
}

struct ConfigurePlugin {
    plugin: Path,
}

impl Parse for ConfigurePlugin {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self { plugin: input.parse()? })
    }
}

#[proc_macro_attribute]
pub fn configure_plugin(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let func_name = &input.sig.ident;

    let plugin = parse_macro_input!(attr as ConfigurePlugin).plugin;

    quote! {
        #input

        ::bevy_butler::__internal::inventory::submit! {
            ::bevy_butler::__internal::ButlerFunc::new::<#plugin>(#func_name)
        }
    }.into()
}