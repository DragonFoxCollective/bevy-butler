use darling::{util::Flag, FromMeta};
use syn::Path;

#[derive(Debug, FromMeta)]
pub(crate) struct AddToGroupAttr {
    pub group: Path,
    pub as_group: Flag,
}
