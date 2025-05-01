use deluxe::ParseMetaItem;
use syn::Expr;

#[derive(ParseMetaItem)]
pub(crate) struct ButlerPluginGroupAttr {
    pub name: Option<Expr>,
}
