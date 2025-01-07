use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::{parse, Error, FnArg, Ident, ImplItem, ImplItemFn, ItemImpl, ItemStruct, Meta, Pat, Path, PathSegment, Type};

use crate::utils::get_crate;

fn butler_plugin_block(app_ident: &Ident, bevy_butler: &Path, plugin: &Path) -> TokenStream {
    quote! {{
        let funcs = #app_ident.world().get_resource::<#bevy_butler::__internal::ButlerRegistry>()
            .unwrap_or_else(|| panic!("#[butler_plugin] requires that the App contain BevyButlerPlugin!"))
            .get_funcs::<#plugin>();

        let mut _butler_systems = 0;
        if let Some(funcs) = funcs {
            for butler_func in &(*funcs) {
                if let Some(sys) = butler_func.try_get_func::<#plugin>() {
                    (sys)(self, app);
                    _butler_systems += 1;
                }
            }
        }

        #bevy_butler::__internal::_butler_debug(&format!("{} added {_butler_systems} systems", stringify!(#plugin)));
    }}.into()
}

/// Modify an existing `Plugin::build` function and insert our
/// prep code either at the top or the bottom
fn butler_plugin_modify_build(plugin: &Path, bevy_butler: &Path, item_func: &mut ImplItemFn) -> Result<(), TokenStream> {
    let app_ident = match &item_func.sig.inputs[1] {
        FnArg::Typed(pat) => match &(*pat.pat) {
            Pat::Ident(ident) => &ident.ident,
            _ => unreachable!(),
        },
        _ => unreachable!(),
    };

    let butler_block = butler_plugin_block(app_ident, &bevy_butler, plugin);
    item_func.block.stmts.insert(0, syn::parse(butler_block.into()).unwrap());

    Ok(())
}

/// Implementation for impl-style butler-plugin invocations
/// 
/// i.e.
/// ```ignore
/// #[butler_plugin]
/// impl Plugin for TestPlugin {
///     fn build(&self, &mut App)
///     {
///         // Some code
///     }
/// }
/// ```
pub(crate) fn butler_plugin_impl(_args: TokenStream, mut item_impl: ItemImpl) -> TokenStream {
    let bevy_butler: Path = match crate::utils::get_crate("bevy-butler").map_err(|e| {
        let err = Error::new(Span::call_site(), e.to_string()).to_compile_error();
        proc_macro::TokenStream::from(err)
    }) {
        Ok(path) => path,
        Err(e) => return e,
    };

    // Check that the impl block is for the Plugin trait
    if item_impl.trait_.as_ref().unwrap().1.segments.last().unwrap().ident != "Plugin" {
        return Error::new(Span::call_site(), "#[butler_plugin] can only modify `impl Plugin` blocks")
            .into_compile_error()
            .into();
    }
    // We can't fully guarantee that the `Plugin` is actually `bevy::prelude::Plugin`... oh well.

    // Get the struct name
    let plugin = if let Type::Path(plugin) = &(*item_impl.self_ty) {
        &plugin.path
    }
    else {
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
        if let Err(e) = butler_plugin_modify_build(plugin, &bevy_butler, item)
        {
            return e;
        }
    }
    else {
        // There's no build function, inject our own.
        let butler_block: proc_macro2::TokenStream = butler_plugin_block(
            &Ident::new("app", Span::call_site()),
            &bevy_butler,
            plugin
        ).into();
        let build = quote! {
            fn build(&self, app: &mut App) {
                #butler_block
            }
        };
        item_impl.items.push(syn::parse(build.into()).unwrap());
    }

    return item_impl.to_token_stream().into();
}

pub(crate) fn butler_plugin_struct(_args: TokenStream, item_struct: ItemStruct) -> TokenStream {
    let ident = &item_struct.ident;

    let bevy_butler = get_crate("bevy-butler").unwrap();

    let bevy_app = get_crate("bevy")
        .map(|mut name| {name.segments.push(syn::parse_str("app").unwrap()); name})
        .or_else(|_| get_crate("bevy_app"))
        .expect("Failed to find bevy_app");

    let butler_block: proc_macro2::TokenStream = butler_plugin_block(
        &syn::parse_str("app").unwrap(),
        &bevy_butler,
        &Path::from(ident.clone())
    ).into();

    quote! {
        #item_struct

        impl #bevy_app::Plugin for #ident {
            fn build(&self, app: &mut #bevy_app::App) {
                #butler_block
            }
        }
    }.into()
}