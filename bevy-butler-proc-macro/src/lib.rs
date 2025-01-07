use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::{Parse, ParseStream}, parse_macro_input, Expr, ItemFn, ItemStruct, Path, Token};

struct SystemArgs {
    schedule: Path,
    plugin: Path,
    transforms: Option<Expr>,
}

impl Parse for SystemArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let schedule: Path = input.parse()?;
        input.parse::<Token![,]>()?;
        let plugin: Path = input.parse()?;
        
        let transforms: Option<Expr> = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            eprintln!("NEXT THING IS COMMA");
            Some(input.parse()?)
        }
        else {
            None
        };

        Ok(Self {
            schedule,
            plugin,
            transforms: None
        })
    }
}

/// Mark a system to be included in a Schedule by a Plugin.
#[proc_macro_attribute]
pub fn system(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let args = parse_macro_input!(attr as SystemArgs);
    let schedule = args.schedule;
    let plugin = args.plugin;
    let transforms = args.transforms.map(|transforms| quote! { .#transforms });

    let func_name = input.sig.ident.clone();
    let butler_func_name = format_ident!("_butler_{}", func_name);

    quote! {
        #input

        fn #butler_func_name (plugin: &#plugin, app: &mut bevy::prelude::App) {
            app.add_systems(#schedule, #func_name #transforms);
        }

        ::bevy_butler::inventory::submit! {
            ::bevy_butler::ButlerFunc::new::<#plugin>(#butler_func_name)
        }
    }.into()
}

#[proc_macro_attribute]
pub fn auto_plugin(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let plugin = &input.ident;

    quote! {
        #input

        impl Plugin for #plugin {
            fn build(&self, app: &mut bevy::app::App) {
                eprintln!("Building {}", stringify!(#plugin));
                for butler_func in ::bevy_butler::inventory::iter::<bevy_butler::ButlerFunc> {
                    if let Some(sys) = butler_func.try_get_func::<#plugin>() {
                        eprintln!("Adding func {:?}", &sys);
                        (sys)(self, app);
                    }
                }
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

        ::bevy_butler::inventory::submit! {
            ::bevy_butler::ButlerFunc::new::<#plugin>(#func_name)
        }
    }.into()
}