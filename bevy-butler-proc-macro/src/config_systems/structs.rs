use quote::{quote, ToTokens};
use syn::{parse::{discouraged::AnyDelimiter, Parse, ParseStream}, Item};

use crate::system::structs::SystemAttr;

#[derive(Clone, Debug)]
pub(crate) struct ConfigSystemsInput {
    pub system_args: SystemAttr,
    pub items: Vec<Item>,
}

impl Parse for ConfigSystemsInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the default system args in the form (plugin = ..., schedule = ..., etc.)
        let system_args: SystemAttr = {
            // Tried to use parenthesized! but it kept complaining about references, oh well
            let (_, _, parse) = input.parse_any_delimiter()?;
            parse.parse()?
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