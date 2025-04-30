use proc_macro2::Span;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Error, Meta, Token, Path,
};

use crate::utils::{parse_meta_args, parse_meta_args_with};

pub(crate) struct RegisterTypeAttr {
    pub plugin: Option<Path>,
    pub type_data: Vec<Path>,
}

impl RegisterTypeAttr {
    pub fn insert_plugin(&mut self, plugin: Path) -> syn::Result<()> {
        if self.plugin.is_some() {
            return Err(Error::new_spanned(
                plugin,
                "Multiple declarations of \"plugin\"",
            ));
        }

        self.plugin = Some(plugin);
        Ok(())
    }

    pub fn require_plugin(&self) -> syn::Result<&Path> {
        self.plugin.as_ref().ok_or(Error::new(
            Span::call_site(),
            "Expected a defined or inherited `plugin` argument",
        ))
    }
}

impl Parse for RegisterTypeAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ret = Self {
            plugin: None,
            type_data: Vec::new(),
        };

        for meta in input.parse_terminated(Meta::parse, Token![,])? {
            match meta.path().require_ident()? {
                ident if ident == "plugin" => ret.insert_plugin(parse_meta_args(meta)?)?,
                ident if ident == "type_data" => ret.type_data.extend(parse_meta_args_with(
                    Punctuated::<Path, Token![,]>::parse_terminated,
                    meta,
                )?),
                ident => {
                    return Err(Error::new_spanned(
                        ident,
                        format!("Unknown argument \"{}\"", ident),
                    ))
                }
            }
        }

        Ok(ret)
    }
}
