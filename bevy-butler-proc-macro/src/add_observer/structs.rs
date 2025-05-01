use deluxe::ParseMetaItem;
use syn::{Path, AngleBracketedGenericArguments};

#[derive(ParseMetaItem)]
pub(crate) struct ObserverAttr {
    pub plugin: Path,
    pub generics: Option<AngleBracketedGenericArguments>,
}