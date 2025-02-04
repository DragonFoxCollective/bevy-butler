use proc_macro2::Span;
use syn::{parse::{Parse, ParseStream}, Error, Expr, Meta, Token, TypePath};

use crate::utils::parse_meta_args;

pub(crate) enum AddType {
    /// `PluginGroupBuilder.add_before`
    Before(Expr),
    /// `PluginGroupBuilder.add_after`
    After(Expr),
    /// `PluginGroupBuilder.add_group`
    Group,
}

pub(crate) struct AddToGroupAttr {
    pub group: Option<TypePath>,
    pub add_type: Option<AddType>,
}

impl AddToGroupAttr {
    pub fn insert_add_type(&mut self, add_type: AddType) -> syn::Result<()> {
        if self.add_type.is_some() {
            return Err(Error::new(Span::call_site(), "Expected a single declaration of `before`, `after` or `group`"));
        }
        
        self.add_type = Some(add_type);
        Ok(())
    }

    pub fn insert_group(&mut self, group: TypePath) -> syn::Result<()> {
        if self.group.is_some() {
            return Err(Error::new_spanned(group, "Multiple declarations of `group`"));
        }

        self.group = Some(group);
        Ok(())
    }

    pub fn require_group(&self) -> syn::Result<&TypePath> {
        self.group.as_ref().ok_or(Error::new(Span::call_site(), "Requires a `group` argument"))
    }
}

impl Parse for AddToGroupAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut ret = AddToGroupAttr {
            group: None,
            add_type: None,
        };

        for meta in input.parse_terminated(Meta::parse, Token![,])? {
            let ident = meta.path().require_ident()?;
            match ident {
                ident if ident == "before" => {
                    ret.insert_add_type(AddType::Before(parse_meta_args(meta)?))?;
                }
                ident if ident == "after" => {
                    ret.insert_add_type(AddType::After(parse_meta_args(meta)?))?;
                }
                ident if ident == "group" => {
                    ret.insert_group(parse_meta_args(meta)?)?;
                }
                ident if ident == "as_group" => {
                    meta.require_path_only()?;
                    ret.insert_add_type(AddType::Group)?;
                }
                ident => return Err(Error::new_spanned(ident, format!("Unknown argument \"{ident}\""))),
            }
        }

        Ok(ret)
    }
}