use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

pub(crate) mod utils;

pub(crate) mod butler_plugin;

fn result_to_tokens(result: syn::Result<TokenStream2>) -> TokenStream {
    match result {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

#[proc_macro_attribute]
pub fn butler_plugin(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(butler_plugin::macro_impl(attr, body))
}

pub(crate) mod add_system;
#[proc_macro_attribute]
pub fn add_system(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(add_system::macro_impl(attr, body))
}

pub(crate) mod add_observer;
#[proc_macro_attribute]
pub fn add_observer(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(add_observer::macro_impl(attr, body))
}

pub(crate) mod insert_resource;
#[proc_macro_attribute]
pub fn insert_resource(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(insert_resource::macro_impl(attr, body))
}

pub(crate) mod add_event;
#[proc_macro_attribute]
pub fn add_event(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(add_event::macro_impl(attr, body))
}

pub(crate) mod register_type;
#[proc_macro_attribute]
pub fn register_type(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(register_type::macro_impl(attr, body))
}

pub(crate) mod butler_plugin_group;
#[proc_macro_attribute]
pub fn butler_plugin_group(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(butler_plugin_group::macro_impl(attr, body))
}

pub(crate) mod add_plugin;
#[proc_macro_attribute]
pub fn add_plugin(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(add_plugin::macro_impl(attr, body))
}

pub(crate) mod add_plugin_group;
#[proc_macro_attribute]
pub fn add_plugin_group(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(add_plugin_group::macro_impl(attr, body))
}

pub(crate) mod insert_state;
#[proc_macro_attribute]
pub fn insert_state(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(insert_state::macro_impl(attr, body))
}
