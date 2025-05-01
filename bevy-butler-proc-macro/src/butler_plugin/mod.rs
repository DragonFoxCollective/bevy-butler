use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_quote, Error, FnArg, Ident, ImplItem, Item, ItemEnum, ItemImpl, ItemStruct, Pat, TypePath,
};

#[derive(deluxe::ParseMetaItem)]
pub struct ButlerPluginAttr;

pub(crate) fn macro_impl(attr: TokenStream1, item: TokenStream1) -> syn::Result<TokenStream2> {
    let attr: ButlerPluginAttr = deluxe::parse(attr)?;
    let item = syn::parse::<Item>(item)?;
    match item {
        Item::Impl(i_impl) => impl_impl(attr, i_impl),
        Item::Struct(i_struct) => struct_impl(attr, i_struct),
        Item::Enum(i_enum) => enum_impl(attr, i_enum),
        other => Err(Error::new_spanned(
            other,
            "Expected `struct`, `enum` or `impl Plugin`",
        )),
    }
}

fn register_butler_plugin_stmts(plugin: &TypePath) -> TokenStream2 {
    quote! {
        impl #plugin {
            pub(crate) fn _butler_plugin_sealed_marker() -> ::std::any::TypeId {
                struct SealedMarker;

                ::std::any::TypeId::of::<SealedMarker>()
            }
        }

        impl ::bevy_butler::__internal::ButlerPlugin for #plugin {}
    }
}

pub(crate) fn struct_impl(_attr: ButlerPluginAttr, item: ItemStruct) -> syn::Result<TokenStream2> {
    let impl_block = impl_plugin_block(_attr, &item.ident)?;

    Ok(quote! {
        #item

        #impl_block
    })
}

pub(crate) fn enum_impl(_attr: ButlerPluginAttr, item: ItemEnum) -> syn::Result<TokenStream2> {
    let impl_block = impl_plugin_block(_attr, &item.ident)?;

    Ok(quote! {
        #item

        #impl_block
    })
}

pub(crate) fn impl_plugin_block(
    _attr: ButlerPluginAttr,
    ident: &Ident,
) -> syn::Result<TokenStream2> {
    let register_block = register_butler_plugin_stmts(&syn::parse2(quote!(#ident))?);

    Ok(quote! {
        impl ::bevy_butler::__internal::bevy_app::Plugin for #ident {
            fn build(&self, app: &mut ::bevy_butler::__internal::bevy_app::App) {
                <Self as ::bevy_butler::__internal::ButlerPlugin>::register_butler_systems(app, Self::_butler_plugin_sealed_marker());
            }
        }

        #register_block
    })
}

pub(crate) fn impl_impl(_attr: ButlerPluginAttr, mut body: ItemImpl) -> syn::Result<TokenStream2> {
    let register_block = |app_ident: &Ident| {
        syn::parse2(quote!(
            <Self as ::bevy_butler::__internal::ButlerPlugin>::register_butler_systems(#app_ident, Self::_butler_plugin_sealed_marker());
        ))
    };

    let build_index = body.items.iter().position(|i| {
        if let ImplItem::Fn(item) = i {
            return item.sig.ident == "build";
        }
        false
    });

    if let Some(build_index) = build_index {
        // Insert butler statement into build func
        let ImplItem::Fn(build) = &mut body.items[build_index] else {
            unreachable!();
        };

        // Figure out the identifier of the `&mut App` argument
        let app_ident = build
            .sig
            .inputs
            .get(1)
            .ok_or(Error::new_spanned(&build.sig, "Missing `app` argument?"))?;
        let app_ident = match app_ident {
            FnArg::Typed(ident) => match &*ident.pat {
                Pat::Ident(ident) => &ident.ident,
                other => return Err(Error::new_spanned(other, "Expected `app: &mut App`")),
            },
            FnArg::Receiver(r) => return Err(Error::new_spanned(r, "Receiver arg in arg 1????")),
        };

        // Insert our registration step into the beginning
        build.block.stmts.insert(0, register_block(app_ident)?);
    } else {
        // No build statement, insert it ourselves
        let register = register_block(&format_ident!("app"))?;
        body.items.push(parse_quote! {
            fn build(&self, app: &mut ::bevy_butler::__internal::bevy_app::App) {
                #register
            }
        });
    }

    let plugin = &body.self_ty;

    let register_block = register_butler_plugin_stmts(&syn::parse2(quote!(#plugin))?);

    Ok(quote! {
        #body

        #register_block
    })
}
