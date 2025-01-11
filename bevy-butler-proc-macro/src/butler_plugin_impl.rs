//! This file enables #[butler_plugin] to be used in two ways
//!
//! 1. Attaching it to a struct definition will generate an `impl Plugin` for it
//!
//! 2. Attaching it to an `impl Plugin` definition will add a hook at the beginning
//!    of the `build` function, or create the `build` function if one isn't present.

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote, ToTokens};
use syn::{
    spanned::Spanned, Error, Expr, ExprCall, FnArg, Ident, ImplItem, ImplItemFn, ItemImpl,
    ItemStruct, Pat, Path, Stmt, Token, Type,
};

use crate::utils::get_crate;

/// Butler initialization function injected at the start of a Plugin's `build` function
fn butler_plugin_block(app_ident: &Ident, bevy_butler: &Path) -> ExprCall {
    syn::parse2(quote! {
        <Self as #bevy_butler::__internal::ButlerPlugin>::register_butler_plugins(#app_ident)
    })
    .expect("Failed to parse butler initialization block")
}

/// `impl ButlerPlugin` block for a Plugin
fn impl_butler_plugin_block(plugin: &Path, bevy_butler: &Path) -> proc_macro2::TokenStream {
    let ident = plugin.segments.last().unwrap().ident.clone();
    let marker_mod_name = format_ident!(
        "_butler_marker_{}",
        &sha256::digest(plugin.to_token_stream().to_string())
    );
    let marker_name = format_ident!("{}CrateAccessMarker", ident);

    quote! {
        mod #marker_mod_name {
            pub struct #marker_name {
                pub(crate) internal_crate_protection_marker: (),
            }

            impl #marker_name {
                pub(crate) fn new() -> Self {
                    Self { internal_crate_protection_marker: () }
                }
            }
        }

        #[diagnostic::do_not_recommend]
        impl #bevy_butler::__internal::ButlerPlugin for #plugin {
            type Marker = #marker_mod_name::#marker_name;

            fn _marker() -> Self::Marker {
                Self::Marker::new()
            }
        }
    }
}

/// Modify an existing `Plugin::build` function and insert our hook at the start
fn butler_plugin_modify_build(
    bevy_butler: &Path,
    item_func: &mut ImplItemFn,
) -> Result<(), TokenStream> {
    let app_ident = match &item_func.sig.inputs[1] {
        FnArg::Typed(pat) => match &(*pat.pat) {
            Pat::Ident(ident) => &ident.ident,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    item_func.block.stmts.insert(
        0,
        Stmt::Expr(
            Expr::Call(butler_plugin_block(app_ident, &bevy_butler)),
            Some(Token![;](item_func.span())),
        ),
    );

    Ok(())
}

/// Implementation for impl-style #[butler-plugin] invocations
pub(crate) fn butler_plugin_impl(_args: TokenStream, mut item_impl: ItemImpl) -> TokenStream {
    let bevy_butler: Path = match crate::utils::get_crate("bevy-butler").map_err(|e| {
        let err = Error::new(Span::call_site(), e.to_string()).to_compile_error();
        proc_macro::TokenStream::from(err)
    }) {
        Ok(path) => path,
        Err(e) => return e,
    };

    // Check that the impl block is for the Plugin trait
    if item_impl
        .trait_
        .as_ref()
        .unwrap()
        .1
        .segments
        .last()
        .unwrap()
        .ident
        != "Plugin"
    {
        return Error::new(
            Span::call_site(),
            "#[butler_plugin] can only modify `impl Plugin` blocks",
        )
        .into_compile_error()
        .into();
    }

    // Get the struct name
    let plugin = if let Type::Path(plugin) = &(*item_impl.self_ty) {
        &plugin.path
    } else {
        panic!("Failed to get plugin identifier");
    };

    // Find the `build` function, if it exists
    if let Some(item) = item_impl.items.iter_mut().find_map(|item| {
        if let ImplItem::Fn(item) = item {
            if item.sig.ident == "build" {
                return Some(item);
            }
        }
        None
    }) {
        // We found an existing build function, modify it.
        if let Err(e) = butler_plugin_modify_build(&bevy_butler, item) {
            return e;
        }
    } else {
        // There's no build function, inject our own.
        let butler_block = butler_plugin_block(&Ident::new("app", Span::call_site()), &bevy_butler);
        let build = quote! {
            fn build(&self, app: &mut App) {
                #butler_block
            }
        };
        item_impl.items.push(syn::parse(build.into()).unwrap());
    }

    let butler_plugin_impl = impl_butler_plugin_block(plugin, &bevy_butler);

    return quote! {
        #item_impl

        #butler_plugin_impl
    }
    .into();
}

/// Implementation for struct-style #[butler-plugin] invocations
///
/// ```
/// # use bevy_butler_proc_macro::butler_plugin;
/// # use bevy::prelude::*;
/// #[butler_plugin]
/// struct MyPlugin;
/// #
/// # fn main() {
/// #   App::new().add_plugins(MyPlugin).run();
/// # }
/// ```
pub(crate) fn butler_plugin_struct(_args: TokenStream, item_struct: ItemStruct) -> TokenStream {
    let ident = &item_struct.ident;

    let bevy_butler = get_crate("bevy-butler");
    if let Err(e) = bevy_butler {
        return Error::new(Span::call_site(), e).to_compile_error().into();
    }
    let bevy_butler = bevy_butler.unwrap();

    let butler_block = butler_plugin_block(&syn::parse_str("app").unwrap(), &bevy_butler);

    let butler_plugin_impl =
        impl_butler_plugin_block(&syn::parse2(ident.to_token_stream()).unwrap(), &bevy_butler);

    quote! {
        #item_struct

        impl #bevy_butler::__internal::bevy_app::Plugin for #ident {
            fn build(&self, app: &mut #bevy_butler::__internal::bevy_app::App) {
                #butler_block
            }
        }

        #butler_plugin_impl
    }
    .into()
}
