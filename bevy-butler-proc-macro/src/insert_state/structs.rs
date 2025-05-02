use deluxe::ParseMetaItem;
use syn::{AngleBracketedGenericArguments, Expr, Path};

#[derive(ParseMetaItem)]
pub struct InsertStateAttr {
    pub plugin: Path,
    pub generics: Option<AngleBracketedGenericArguments>,
    pub init: Option<Expr>,
}