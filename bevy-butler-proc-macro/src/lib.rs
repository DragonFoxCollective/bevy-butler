use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse::{Parse, ParseStream}, parse_macro_input, Expr, ExprPath, Ident, ItemFn, ItemStruct, Meta, Path, Token};
use proc_macro_crate::{crate_name, FoundCrate};

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
                    return Err(input.error(format!("Unknown attribute \"{}\"", &name_value.path.to_token_stream())));
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

    let bevy_app = find_bevy_crate("app", "bevy_app");
    let bevy_butler = find_bevy_butler();

    let args = parse_macro_input!(attr as SystemArgs);
    let schedule = args.schedule;
    let dppath = format!("{}::BevyButlerPlugin", bevy_butler.to_token_stream());
    let default_plugin = syn::parse_str(&dppath).expect(&format!("Failed to find {dppath}"));
    let plugin = args.plugin.unwrap_or(default_plugin);
    let func_name = input.sig.ident.clone();
    let transformed_func = args.transforms
        .map(|transforms| quote! { #func_name.#transforms})
        .unwrap_or_else(|| func_name.clone().into_token_stream());

    let butler_func_name = format_ident!("_butler_{}", func_name);

    

    quote! {
        #input

        fn #butler_func_name (plugin: &#plugin, app: &mut #bevy_app::App) {
            app.add_systems(#schedule, #transformed_func);
        }

        #bevy_butler::__internal::inventory::submit! {
            #bevy_butler::__internal::ButlerFunc::new::<#plugin>(#butler_func_name)
        }
    }.into()
}

fn find_bevy_crate(supercrate: &str, subcrate: &str) -> syn::Path {
    crate_name("bevy").map(|found|
        match found {
            FoundCrate::Itself => syn::parse_str(&format!("crate::{}", supercrate)).expect("Failed to unwrap self"),
            proc_macro_crate::FoundCrate::Name(name) => {
                syn::parse_str(&format!("::{}::{}", name, supercrate)).expect(&format!("Failed to parse path for ::{}::{}", name, supercrate))
            }
        }
    ).unwrap_or_else(|_| {
        crate_name(subcrate).map(|found| {
            match found {
                FoundCrate::Itself => syn::parse_str("crate").unwrap(),
                FoundCrate::Name(name) => {
                    syn::parse_str(&format!("::{}", &name)).expect(&format!("Failed to parse path for ::{}", name))
                }
            }
        }).expect(&format!("Failed to find bevy::{} or {}", supercrate, subcrate))
    })
}

fn find_bevy_butler() -> syn::Path {
    return crate_name("bevy-butler").map(|found| {
        match found {
            FoundCrate::Itself => syn::parse_str("::bevy_butler").expect("Failed to refer to bevy-butler"),
            FoundCrate::Name(name) => {
                syn::parse_str(&format!("::{}", &name.trim())).unwrap()
            }
        }
    }).expect("Failed to find bevy_butler");
}

#[proc_macro_attribute]
pub fn auto_plugin(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let plugin = &input.ident;

    let bevy_app = find_bevy_crate("app", "bevy_app");
    let bevy_butler: Path = find_bevy_butler();
    eprintln!("PLUGIN PATH: {}", bevy_butler.to_token_stream());

    quote! {
        #input

        impl #bevy_app::Plugin for #plugin {
            fn build(&self, app: &mut #bevy_app::App) {
                let funcs = app.world().get_resource_ref::<#bevy_butler::__internal::ButlerRegistry>()
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
                
                #bevy_butler::__internal::_butler_debug(&format!("{} added {sys_count} systems", stringify!(#plugin)));
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

    let bevy_butler = find_bevy_butler();

    quote! {
        #input

        #bevy_butler::__internal::inventory::submit! {
            #bevy_butler::__internal::ButlerFunc::new::<#plugin>(#func_name)
        }
    }.into()
}