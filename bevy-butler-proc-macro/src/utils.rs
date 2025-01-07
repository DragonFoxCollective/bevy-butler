use proc_macro_crate::{crate_name, FoundCrate};
use syn::Path;

pub(crate) fn get_crate(name: &str) -> Result<Path, proc_macro_crate::Error>
{
    crate_name(name).map(|found| {
        match found {
            FoundCrate::Itself => syn::parse_str(&name.to_string().replace("-", "_")).unwrap(),
            FoundCrate::Name(actual) => syn::parse_str(&format!("::{}", actual)).unwrap(),
        }
    })
}