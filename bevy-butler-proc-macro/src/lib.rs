use proc_macro::TokenStream;

pub(crate) mod butler_plugin;
#[proc_macro_attribute]
pub fn butler_plugin(attr: TokenStream, body: TokenStream) -> TokenStream {
    match butler_plugin::macro_impl(attr, body) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

pub(crate) mod system;
#[proc_macro_attribute]
pub fn system(attr: TokenStream, body: TokenStream) -> TokenStream {
    match system::macro_impl(attr, body) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

pub(crate) mod config_systems;
#[proc_macro]
pub fn config_systems(body: TokenStream) -> TokenStream {
    match config_systems::macro_impl(body) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Dummy attribute to provide defaults from config_system!
#[proc_macro_attribute]
pub fn _butler_config_systems_defaults(_attr: TokenStream, body: TokenStream) -> TokenStream {
    body
}

pub(crate) mod system_set;
#[proc_macro]
pub fn system_set(body: TokenStream) -> TokenStream {
    match system_set::macro_impl(body) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.into_compile_error().into(),
    }
}