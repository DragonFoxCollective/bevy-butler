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

pub(crate) mod config_systems;
#[proc_macro]
pub fn config_systems(body: TokenStream) -> TokenStream {
    result_to_tokens(config_systems::macro_impl(body))
}

pub(crate) mod add_system_set;
#[proc_macro]
pub fn add_system_set(body: TokenStream) -> TokenStream {
    result_to_tokens(add_system_set::macro_impl(body))
}

pub(crate) mod add_observer;
#[proc_macro_attribute]
pub fn add_observer(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(add_observer::macro_impl(attr, body))
}

pub(crate) mod add_resource;
#[proc_macro_attribute]
pub fn add_resource(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(add_resource::macro_impl(attr, body))
}

pub(crate) mod register_event;
#[proc_macro_attribute]
pub fn register_event(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(register_event::macro_impl(attr, body))
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

pub(crate) mod add_to_group;
#[proc_macro_attribute]
pub fn add_to_group(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(add_to_group::macro_impl(attr, body))
}

pub(crate) mod add_to_plugin;
#[proc_macro_attribute]
pub fn add_to_plugin(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(add_to_plugin::macro_impl(attr, body))
}