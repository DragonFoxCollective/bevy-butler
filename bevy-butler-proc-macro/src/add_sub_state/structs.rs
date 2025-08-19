use deluxe::ParseMetaItem;
use syn::{AngleBracketedGenericArguments, Path};

#[derive(ParseMetaItem)]
pub struct AddSubStateAttr {
    pub plugin: Path,
    pub generics: Option<AngleBracketedGenericArguments>,
}