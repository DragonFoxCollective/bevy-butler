use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse::{discouraged::AnyDelimiter, Parse, ParseStream}, parse_macro_input, Ident, ItemFn, ItemStruct, Meta, Path, Token};

struct SystemArgs {
    schedule: Path,
    plugin: Path,
}

impl Parse for SystemArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let schedule: Path = input.parse()?;
        input.parse::<Token![,]>()?;
        let plugin: Path = input.parse()?;

        Ok(Self {
            schedule,
            plugin,
        })
    }
}

#[proc_macro_attribute]
pub fn system(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let args = parse_macro_input!(attr as SystemArgs);
    let schedule = args.schedule;
    let plugin = args.plugin;

    let func_name = input.sig.ident.clone();
    let butler_func_name = format_ident!("_butler_{}", func_name);

    quote! {
        #input

        fn #butler_func_name (app: &mut bevy::prelude::App) {
            println!("ADDING DA FUNCTION");
            app.add_systems(#schedule, #func_name);
        }

        bevy_butler::__internal::inventory::submit! {
            bevy_butler::ButlerSystem::<#plugin> {
                func: #butler_func_name,
                marker: std::marker::PhantomData,
            }
        }
    }.into()
}

#[proc_macro_attribute]
pub fn auto_plugin(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let plugin = &input.ident;

    eprintln!("IDENT: {plugin}");

    quote! {
        #input

        impl Plugin for #plugin {
            fn build(&self, app: &mut bevy::app::App) {
                for butler_sys in bevy_butler::__internal::inventory::iter::<bevy_butler::ButlerSystem<#plugin>> {
                    (butler_sys.func)(app);
                }
            }
        }
    }.into()
}