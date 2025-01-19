use proc_macro::TokenStream;

mod butler_plugin;
#[proc_macro_attribute]
pub fn butler_plugin(attr: TokenStream, body: TokenStream) -> TokenStream {
    match butler_plugin::macro_impl(attr, body) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

mod system;
#[proc_macro_attribute]
pub fn system(attr: TokenStream, body: TokenStream) -> TokenStream {
    match system::macro_impl(attr, body) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

mod config_systems;
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