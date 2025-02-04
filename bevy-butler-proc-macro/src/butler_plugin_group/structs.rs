use syn::{parse::{Parse, ParseStream}, Error, Expr, Meta, Token};

use crate::utils::parse_meta_args;

pub(crate) struct ButlerPluginGroupAttr {
    pub name: Option<Expr>,
}

impl ButlerPluginGroupAttr {
    pub fn insert_name(&mut self, name: Expr) -> syn::Result<()> {
        if self.name.is_some() {
            return Err(Error::new_spanned(name, "Multiple declarations of `name`"));
        }

        self.name = Some(name);
        Ok(())
    }
}

impl Parse for ButlerPluginGroupAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self { name: None });
        }

        let mut ret = Self {
            name: None,
        };

        for meta in input.parse_terminated(Meta::parse, Token![,])? {
            let ident = meta.path().require_ident()?.clone();
            if ident != "name" {
                return Err(Error::new_spanned(meta, format!("Unknown argument \"{ident}\"")));
            }
            let name_expr: Expr = parse_meta_args(meta)?;
            ret.insert_name(name_expr)?;
        }

        Ok(ret)
    }
}