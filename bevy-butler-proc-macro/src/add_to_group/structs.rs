use deluxe::{Flag, ParseMetaItem};
use syn::Path;

#[derive(Debug, ParseMetaItem)]
pub(crate) struct AddToGroupAttr {
    pub group: Path,
    pub as_group: Flag,
}
