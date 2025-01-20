use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use structs::ConfigSystemsInput;
use syn::{parse::{Parse, Parser}, Attribute, Item, MetaList};

pub mod structs;

pub(crate) const CONFIG_SYSTEMS_DEFAULT_ARGS_IDENT: &'static str = "_butler_config_systems_defaults";

/// Process a `config_systems!` invocation and return the resulting transformed items
pub(crate) fn parse_config_systems(input: ConfigSystemsInput) -> syn::Result<Vec<Item>> {
    let defaults = input.system_args;
    let items = input.items;

    let arg_metas = defaults.get_metas();
    let config_attr: Attribute = Attribute {
        pound_token: Default::default(),
        style: syn::AttrStyle::Outer,
        bracket_token: Default::default(),
        meta: syn::Meta::List(MetaList {
            path: syn::parse_str(&CONFIG_SYSTEMS_DEFAULT_ARGS_IDENT)?,
            delimiter: syn::MacroDelimiter::Brace(Default::default()),
            tokens: arg_metas.to_token_stream(),
        })
    };

    // For every item, insert the dummy attribute `_butler_config_systems_default`
    // Systems will read from this attribute and apply the default arguments to their
    // own arguments.
    //
    // Any non-systems will simply ignore the attribute.
    Ok(items.into_iter().try_fold(vec![], |mut items, item| {
        match item {
            // Could be a system with `#[system]`
            Item::Fn(mut item_fn) => {
                item_fn.attrs.push(config_attr.clone());
                items.push(Item::Fn(item_fn));
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
                    items.push(Item::Macro(item_macro));
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