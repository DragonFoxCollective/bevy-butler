use proc_macro2::Span;
use syn::{parse::{Parse, ParseStream}, Error, Meta, Token, TypePath};

use crate::utils::parse_meta_args;

pub(crate) struct EventAttr {
    pub plugin: Option<TypePath>,
}

impl EventAttr {
    pub fn insert_plugin(&mut self, plugin: TypePath) -> syn::Result<()> {
        if self.plugin.is_some() {
            return Err(Error::new_spanned(plugin, "Multiple declarations of \"plugin\""));
        }

        self.plugin = Some(plugin);
        Ok(())
    }
    
    pub fn require_plugin(&self) -> syn::Result<&TypePath> {
        self.plugin.as_ref().ok_or(Error::new(Span::call_site(), "Expected a defined or inherited `plugin` argument"))
    }
}

impl Parse for EventAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ret = EventAttr {
            plugin: None,
        };

        for meta in input.parse_terminated(Meta::parse, Token![,])? {
            match meta.path().require_ident()? {
                ident if ident == "plugin" => ret.insert_plugin(parse_meta_args::<TypePath>(meta)?)?,
                ident => return Err(Error::new_spanned(ident, format!("Unknown argument \"{}\"", ident))),
            }
        }

        Ok(ret)
    }
}
