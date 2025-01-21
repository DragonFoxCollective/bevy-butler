use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use structs::{ButlerPluginAttr, ButlerPluginInput, PluginStage, PluginStageData};
use syn::{parse::Parser, spanned::Spanned, Error, FnArg, ImplItem, ItemImpl, ItemStruct, Pat, TypePath};

pub mod structs;

pub(crate) fn macro_impl(attr: TokenStream1, item: TokenStream1) -> syn::Result<TokenStream2> {
    let attr = ButlerPluginAttr::parse_inner.parse(attr)?;
    let input = ButlerPluginInput::parse_with_attr(attr).parse(item)?;

    match input {
        ButlerPluginInput::Struct { attr, body } => struct_impl(attr, body),
        ButlerPluginInput::Impl { attr, body } => impl_impl(attr, body),
    }
}

fn register_butler_plugin_stmts(plugin: &TypePath) -> TokenStream2 {
    quote! {
        impl #plugin {
            pub(crate) fn _butler_sealed_marker() -> ::std::any::TypeId {
                struct SealedMarker;

                ::std::any::TypeId::of::<SealedMarker>()
            }
        }

        impl ::bevy_butler::__internal::ButlerPlugin for #plugin {}
    }
}

pub(crate) fn struct_impl(mut attr: ButlerPluginAttr, body: ItemStruct) -> syn::Result<TokenStream2> {
    let plugin_struct = &body.ident;
    let app_ident = format_ident!("app");
    let build_body = attr.stages[PluginStage::Build as usize].take()
        .map(|data| data.stage_inner_block(&app_ident));
    let fn_iter = attr.stages.into_iter()
        .filter_map(|data| data.map(|d| d.stage_fn()));

    let register_block = register_butler_plugin_stmts(&syn::parse2(quote!(#plugin_struct))?);

    Ok(quote! {
        #body

        impl ::bevy_butler::__internal::bevy_app::Plugin for #plugin_struct {
            fn build(&self, app: &mut ::bevy_butler::__internal::bevy_app::App) {
                <Self as ::bevy_butler::__internal::ButlerPlugin>::register_butler_systems(app, Self::_butler_sealed_marker());
                #build_body
            }

            #(#fn_iter)*
        }

        #register_block
    })
}

pub(crate) fn impl_impl(mut attr: ButlerPluginAttr, mut body: ItemImpl) -> syn::Result<TokenStream2> {
    // Insert a dummy build stage if it doesn't already exist
    // Simplifies the logic for inserting our butler registration function
    attr.stages[PluginStage::Build as usize].get_or_insert(PluginStageData {
        stage: PluginStage::Build,
        attr_span: Span::call_site(),
        stmts: vec![]
    });

    // Insert butler statements into existing blocks
    for item in body.items.iter_mut() {
        if let ImplItem::Fn(item) = item {
            if let Ok(stage) = PluginStage::try_from(&item.sig.ident) {
                // We're in a plugin stage
                // Try to grab the stage data from the attr
                if let Some(data) = attr.stages[stage as usize].take() {
                    // Figure out the identifier of the `&mut App` argument
                    let app_ident = item.sig.inputs.get(1)
                        .ok_or(Error::new(item.sig.span(), "Missing `app` argument?"))?;
                    let app_ident = match app_ident {
                        FnArg::Typed(ident) => match &*ident.pat {
                            Pat::Ident(ident) => &ident.ident,
                            other => return Err(Error::new_spanned(other, "Expected `app: &mut App`")),
                        }
                        FnArg::Receiver(r) => return Err(Error::new_spanned(r, "Receiver arg in arg 1????")),
                    };

                    let stmts = data.stmts;
                    let butler_stmt = syn::parse2(quote! {{
                        #(#app_ident . #stmts;)*
                    }})?;

                    // Insert the stage data into the function
                    item.block.stmts.insert(0, butler_stmt);

                    // If this is the build stage, insert our registration step into the beginning
                    if stage == PluginStage::Build {
                        item.block.stmts.insert(0, syn::parse2(quote!(
                            <Self as ::bevy_butler::__internal::ButlerPlugin>::register_butler_systems(#app_ident, Self::_butler_sealed_marker());
                        ))?);
                    }
                }
            }
        }
    }

    // Create new function blocks for stages that weren't user-defined,
    // but have butler statements
    body.items.extend(
        attr.stages.into_iter()
            .filter_map(|d| {
                d.map(|data| ImplItem::Fn(data.stage_fn()))
            })
    );

    let plugin = &body.self_ty;

    let register_block = register_butler_plugin_stmts(&syn::parse2(quote!(#plugin))?);

    Ok(quote! {
        #body

        #register_block
    })
}