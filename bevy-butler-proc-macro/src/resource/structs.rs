use proc_macro2::Span;
use syn::{parse::{Parse, ParseStream}, Error, Expr, LitBool, Meta, Token, TypePath};

use crate::utils::parse_meta_args;

pub(crate) struct ResourceAttr {
    pub plugin: Option<TypePath>,
    pub init: Option<Expr>,
    pub non_send: Option<bool>,
}

impl ResourceAttr {
    pub fn insert_plugin(&mut self, plugin: TypePath) -> syn::Result<()> {
        if self.plugin.is_some() {
            return Err(Error::new_spanned(plugin, "Multiple declarations of \"plugin\""));
        }

        self.plugin = Some(plugin);
        Ok(())
    }

    pub fn insert_init(&mut self, init: Expr) -> syn::Result<()> {
        if self.init.is_some() {
            return Err(Error::new_spanned(init, "Multiple declarations of \"init\""));
        }

        self.init = Some(init);
        Ok(())
    }

    pub fn insert_non_send(&mut self, non_send: bool) -> syn::Result<()> {
        if self.non_send.is_some() {
            return Err(Error::new(Span::call_site(), "Multiple declarations of \"non_send\""));
        }

        self.non_send = Some(non_send);
        Ok(())
    }
    
    pub fn require_plugin(&self) -> syn::Result<&TypePath> {
        self.plugin.as_ref().ok_or(Error::new(Span::call_site(), "Expected a defined or inherited `plugin` argument"))
    }
}

impl Parse for ResourceAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ret = ResourceAttr {
            plugin: None,
            init: None,
            non_send: None,
        };

        for meta in input.parse_terminated(Meta::parse, Token![,])? {
            match meta.path().require_ident()? {
                ident if ident == "plugin" => ret.insert_plugin(parse_meta_args::<TypePath>(meta)?)?,
                ident if ident == "init" => ret.insert_init(parse_meta_args::<Expr>(meta)?)?,
                ident if ident == "non_send" => match meta {
                    Meta::Path(_) => ret.insert_non_send(true)?,
                    _ => ret.insert_non_send(parse_meta_args::<LitBool>(meta)?.value)?,
                },
                ident => return Err(Error::new_spanned(ident, format!("Unknown argument \"{}\"", ident))),
            }
        }

        Ok(ret)
    }
}
