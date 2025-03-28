use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::SystemSetInput;
use syn::{
    parse::{Parse, Parser},
    punctuated::Punctuated,
    spanned::Spanned,
    Error, Expr, Item, Token,
};

use crate::{
    config_systems::{parse_config_systems, structs::ConfigSystemsInput},
    add_system::structs::SystemAttr,
    utils::butler_plugin_entry_block,
};

pub mod structs;

pub(crate) fn parse_system_set(mut input: SystemSetInput) -> syn::Result<(SystemSetInput, Expr)> {
    let set_args = &input.system_args;
    let items = &mut input.items;

    let set_transforms = set_args.transforms.iter();
    if set_args.generics.is_some() {
        return Err(Error::new(
            set_args.attr_span,
            "`generics` is not applicable for `add_system_set!` arguments",
        ));
    }

    let mut systems: Punctuated<Expr, Token![,]> = Default::default();

    // We are going to manually parse every system and handle its
    // attribute ourselves, instead of letting #[add_system] register
    // it. Then we'll create a single registry entry to register our
    // transformed set.
    let mut i = 0;
    while i < items.len() {
        let item = &mut items[i];
        match item {
            Item::Fn(item_fn) => {
                // extract_if is still unstable, so we gotta get a little icky with the code
                {
                    let mut j = 0;
                    let attrs = &mut item_fn.attrs;
                    while j < attrs.len() {
                        if let Some(sys_args) = SystemAttr::try_parse_system_attr(&attrs[j])? {
                            let sys_attr = attrs.remove(j);
                            // We'll wrap every system into a single set, so no overriding
                            // the plugin/schedule
                            if sys_args.plugin.is_some() {
                                return Err(Error::new(
                                    sys_attr.span(),
                                    "`plugin` can not be overridden within a `add_system_set!` block",
                                ));
                            }
                            if sys_args.schedule.is_some() {
                                return Err(Error::new(
                                    sys_attr.span(),
                                    "`schedule` can not be overridden within a `add_system_set!` block",
                                ));
                            }
                            let fn_ident = &item_fn.sig.ident;
                            let generics = sys_args.generics;
                            let transforms = sys_args.transforms.into_iter();
                            systems
                                .push(syn::parse2(quote!(#fn_ident #generics #(. #transforms)*))?);
                        } else {
                            j += 1;
                        }
                    }
                    i += 1;
                }
            }
            Item::Macro(mac) => {
                match mac.mac.path.get_ident().cloned() {
                    // Nested `add_system_set!`
                    Some(ident) if ident == "add_system_set" => {
                        let mut mac_body: SystemSetInput = mac.mac.parse_body()?;
                        let sys_args = &mut mac_body.system_args;
                        if sys_args.plugin.is_some() {
                            return Err(Error::new(
                                mac.span(),
                                "`plugin` can not be overridden within a `add_system_set!` block",
                            ));
                        }
                        if sys_args.schedule.is_some() {
                            return Err(Error::new(
                                mac.span(),
                                "`schedule` can not be overridden within a `add_system_set!` block",
                            ));
                        }
                        sys_args.with_defaults(set_args.clone());
                        let (mac_body, set_expr) = parse_system_set(mac_body)?;

                        let items_len = mac_body.items.len();

                        items.splice(i..=i, mac_body.items).for_each(|_| ());
                        i += items_len;
                        systems.push(set_expr);
                    }

                    // Nested `config_systems!`
                    Some(ident) if ident == "config_systems" => {
                        let mut mac_body: ConfigSystemsInput = mac.mac.parse_body()?;
                        let sys_args = &mut mac_body.system_args;
                        if sys_args.plugin.is_some() {
                            return Err(Error::new(
                                mac.span(),
                                "`plugin` can not be overridden within a `add_system_set!` block",
                            ));
                        }
                        if sys_args.schedule.is_some() {
                            return Err(Error::new(
                                mac.span(),
                                "`schedule` can not be overridden within a `add_system_set!` block",
                            ));
                        }

                        items
                            .splice(i..=i, parse_config_systems(mac_body)?)
                            .for_each(|_| ());
                    }
                    _ => i += 1,
                }
            }
            _ => i += 1,
        }
    }

    // Construct the system set as an Expr
    let add_system_set: Expr = syn::parse2(quote!( (#systems) #(. #set_transforms)* ))?;

    Ok((input, add_system_set))
}

pub(crate) fn macro_impl(body: TokenStream1) -> syn::Result<TokenStream2> {
    let input = SystemSetInput::parse.parse(body)?;

    // Early terminate
    input.system_args.require_plugin()?;
    input.system_args.require_schedule()?;

    let (input, set_expr) = parse_system_set(input)?;
    let args = input.system_args;

    let plugin = args.require_plugin()?;
    let schedule = args.require_schedule()?;

    // Hash the system set to get a static ident
    #[allow(unused)]
    let set_hash = sha256::digest(format!(
        "{}{}{}",
        plugin.to_token_stream(),
        schedule.to_token_stream(),
        set_expr.clone().to_token_stream(),
    ));

    let static_ident = format_ident!("_butler_sys_set_{}", set_hash);

    let register_block = butler_plugin_entry_block(
        &static_ident,
        plugin,
        &syn::parse_quote! {
            |app| { app.add_systems(#schedule, #set_expr); }
        },
    );

    let items = input.items;

    Ok(quote! {
        #(#items)*

        #register_block
    })
}
