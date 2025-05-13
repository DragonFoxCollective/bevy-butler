use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote, ToTokens};
use structs::SystemAttr;
use syn::{Expr, ExprCall};
use syn::{Ident, Item};

use crate::utils::{butler_plugin_entry_block, get_fn_ident};

pub mod structs;

pub(crate) fn parse_system(attr: &SystemAttr, ident: &Ident) -> Expr {
    let generics = attr.generics.clone().map(|mut g| {
        g.colon2_token = Some(Default::default());
        g
    });
    let transforms = &attr.transforms.0;

    println!("PARSING SYS_EXPR");

    let sys_expr: Expr = syn::parse_quote! {
        #ident #generics #(. #transforms)*
    };

    println!("SYS_EXPR PARSED: {}", sys_expr.to_token_stream());

    match &attr.pipe_in {
        Some(pipes) if !pipes.is_empty() => {
            let mut iter = pipes.iter();
            let first = iter.next().unwrap();
            syn::parse_quote! {
                #first #(.pipe(#iter))* .pipe(#sys_expr)
            }
        }
        _ => sys_expr,
    }
}

pub(crate) fn macro_impl(attr: TokenStream1, item: TokenStream1) -> syn::Result<TokenStream2> {
    let attr = deluxe::parse(attr);
    if let Err(e) = &attr {
        println!("SYSTEM ATTR ERROR: {e:?}");
        return Err(e.clone());
    }
    let attr: SystemAttr = attr.unwrap();
    println!("SYSTEM ATTR PARSED");
    let input: Item = syn::parse(item)?;

    let sys_ident = get_fn_ident(&input)?;

    let plugin = &attr.plugin;
    let schedule = &attr.schedule;

    let sys_expr = parse_system(&attr, sys_ident);

    let mut hash_bytes = "system".to_string();
    hash_bytes += &plugin.to_token_stream().to_string();
    hash_bytes += &schedule.to_token_stream().to_string();
    hash_bytes += &sys_expr.to_token_stream().to_string();
    #[allow(unused_variables)] // It's actually used
    let static_ident = format_ident!("_butler_system_{}", sha256::digest(hash_bytes));

    let register_block = butler_plugin_entry_block(
        &static_ident,
        plugin,
        &syn::parse_quote! {
            |app| { app.add_systems( #schedule, #sys_expr ); }
        },
    );

    Ok(quote! {
        #input

        #register_block
    })
}
