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

pub(crate) mod system;
#[proc_macro_attribute]
pub fn system(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(system::macro_impl(attr, body))
}

pub(crate) mod config_systems;
#[proc_macro]
pub fn config_systems(body: TokenStream) -> TokenStream {
    result_to_tokens(config_systems::macro_impl(body))
}

pub(crate) mod system_set;
#[proc_macro]
pub fn system_set(body: TokenStream) -> TokenStream {
    result_to_tokens(system_set::macro_impl(body))
}

pub(crate) mod observer;
#[proc_macro_attribute]
pub fn observer(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(observer::macro_impl(attr, body))
}

pub(crate) mod resource;
#[proc_macro_attribute]
pub fn resource(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(resource::macro_impl(attr, body))
}

pub(crate) mod event;
#[proc_macro_attribute]
pub fn event(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(event::macro_impl(attr, body))
}

pub(crate) mod register_type;
#[proc_macro_attribute]
pub fn register_type(attr: TokenStream, body: TokenStream) -> TokenStream {
    result_to_tokens(register_type::macro_impl(attr, body))
}
