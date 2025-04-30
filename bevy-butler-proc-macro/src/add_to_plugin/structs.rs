use deluxe::ParseMetaItem;
use syn::{ AngleBracketedGenericArguments, Generics, Path, TypePath };

#[derive(Debug, ParseMetaItem)]
pub(crate) struct AddPluginAttr {
    pub plugin: Path,
    pub generics: Option<AngleBracketedGenericArguments>,
}
