#![feature(proc_macro_span)]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::{Parse, ParseStream}, parse_macro_input, Error, Item, ItemFn, Path};
use proc_macro_crate::{crate_name, FoundCrate};

mod butler_plugin_impl;
mod utils;

#[proc_macro_attribute]
pub fn butler_plugin(args: TokenStream, item: TokenStream) -> TokenStream
{
    let parsed: Item = parse_macro_input!(item as Item);

    match parsed {
        Item::Impl(item_impl) => butler_plugin_impl::butler_plugin_impl(args, item_impl),
        Item::Struct(item_struct) => butler_plugin_impl::butler_plugin_struct(args, item_struct),
        
        _ => Error::new_spanned(
            parsed,
            "#[butler_plugin] can only be invoked on structs or `impl Plugin` blocks."
        )
            .to_compile_error()
            .into()
    }
}

mod system_impl;

#[proc_macro_attribute]
pub fn system(attr: TokenStream, item: TokenStream) -> TokenStream {
    system_impl::system_free_standing_impl(attr, parse_macro_input!(item as ItemFn))
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