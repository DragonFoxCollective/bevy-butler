#![feature(proc_macro_span)]

use proc_macro::TokenStream;
use syn::{parse_macro_input, Error, Item, ItemFn};

mod utils;

mod butler_plugin_impl;
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