use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use structs::ConfigSystemsInput;
use syn::{parse::{Parse, Parser}, Item, Meta};

use crate::system::structs::SystemAttr;

pub mod structs;

/// Process a `config_systems!` invocation and return the resulting transformed items
pub(crate) fn parse_config_systems(input: ConfigSystemsInput) -> syn::Result<Vec<Item>> {
    let defaults = input.system_args;
    let items = input.items;

    // For every system, parse the attr and rewrite it with new defaults
    //
    // Any non-systems will simply ignore the attribute.
    Ok(items.into_iter().try_fold(vec![], |mut items, item| {
        match item {
            // Could be a system with `#[system]`
            Item::Fn(mut item_fn) => {
                for attr in item_fn.attrs.iter_mut() {
                    if let Some(mut sys_attr) = SystemAttr::try_parse_system_attr(attr)? {
                        sys_attr.with_defaults(defaults.clone());
                        attr.meta = Meta::from(sys_attr);
                        eprintln!("Rewritten meta: {}", attr.to_token_stream().to_string());
                    }
                }
                items.push(item_fn.into());
            }
            // Could be `config_systems!`
            Item::Macro(item_macro) => {
                // Regular proc_macros can't read attributes applied, so we actually have to unwrap the macro
                if item_macro.mac.path.get_ident().is_some_and(|i| i == "config_systems") {
                    let mut input: ConfigSystemsInput = item_macro.mac.parse_body()?;
                    input.system_args.with_defaults(defaults.clone());
                    items.extend(parse_config_systems(input)?);
                }
                else {
                    items.push(item_macro.into());
                }
            },
            _ => (),
        }
        syn::Result::Ok(items)
    })?)
}

pub(crate) fn macro_impl(body: TokenStream1) -> syn::Result<TokenStream2> {
    let input = ConfigSystemsInput::parse.parse(body)?;
    
    let items = parse_config_systems(input)?;

    Ok(quote! {
        #(#items)*
    })
}