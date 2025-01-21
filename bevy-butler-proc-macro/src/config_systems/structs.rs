use quote::{quote, ToTokens};
use syn::{parse::{discouraged::{AnyDelimiter, Speculative}, Parse, ParseStream}, Item};

use crate::system::structs::SystemAttr;

#[derive(Clone)]
pub(crate) struct ConfigSystemsInput {
    pub system_args: SystemAttr,
    pub items: Vec<Item>,
}

impl Parse for ConfigSystemsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the default system args in the form (plugin = ..., schedule = ..., etc.)
        let fork = input.fork();
        let system_args = if let Ok((_, _, parse)) = fork.parse_any_delimiter() {
            input.advance_to(&fork);
            parse.parse()?
        }
        else {
            SystemAttr::default()
        };

        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?)
        }

        Ok(Self {
            system_args,
            items
        })
    }
}

impl ToTokens for ConfigSystemsInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let args = self.system_args.get_metas();
        let items = &self.items;
        tokens.extend(quote! {
            (#args)

            #(#items)*
        });
    }
}