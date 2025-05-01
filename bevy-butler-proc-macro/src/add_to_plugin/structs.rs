use deluxe::ParseMetaItem;
use syn::Path;

#[derive(Debug, ParseMetaItem)]
pub(crate) struct AddPluginAttr {
    pub plugin: Path,
}
