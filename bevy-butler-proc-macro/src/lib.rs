use proc_macro::TokenStream;

mod butler_plugin;
#[proc_macro_attribute]
pub fn butler_plugin(attr: TokenStream, body: TokenStream) -> TokenStream {
    match butler_plugin::macro_impl(attr, body) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}