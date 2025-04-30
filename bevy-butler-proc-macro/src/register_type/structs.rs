use deluxe::ParseMetaItem;
use syn::Path;

#[derive(ParseMetaItem)]
pub(crate) struct RegisterTypeAttr {
    pub plugin: Path,
    #[deluxe(default)]
    pub type_data: Vec<Path>,
}
