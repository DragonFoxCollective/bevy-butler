use deluxe::{Flag, ParseMetaItem};
use syn::{AngleBracketedGenericArguments, Expr, Path};

#[derive(ParseMetaItem)]
pub(crate) struct ResourceAttr {
    pub plugin: Path,
    pub init: Option<Expr>,
    pub non_send: Flag,
    pub generics: Option<AngleBracketedGenericArguments>,
}
