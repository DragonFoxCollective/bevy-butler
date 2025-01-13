//! This file enables #[butler_plugin] to be used in two ways
//!
//! 1. Attaching it to a struct definition will generate an `impl Plugin` for it
//!
//! 2. Attaching it to an `impl Plugin` definition will add a hook at the beginning
//!    of the `build` function, or create the `build` function if one isn't present.

use std::collections::HashSet;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Parser},
    punctuated::Punctuated,
    Attribute, Error, Expr, FnArg, Ident, ImplItem, Item, ItemImpl, ItemStruct, Meta, Pat, Type,
};

pub(crate) type PluginStageOps = Vec<Expr>;
pub(crate) type PluginStageData = [Option<PluginStageOps>; 4];

pub(crate) enum ButlerPluginInput {
    /// ```ignore
    /// #[butler_plugin]
    /// struct MyPlugin;
    /// ```
    Struct(ItemStruct, PluginStageData),

    ///```ignore
    /// #[butler_plugin]
    /// impl Plugin for MyPlugin {
    ///     /* Stuff */
    /// }
    /// ```
    Impl(ItemImpl, PluginStageData),
}

/// ```ignore
/// #[butler_plugin]
/// #[build(/* Some app.* functions */)]
/// #[ready(/* Some app.* functions */)]
/// #[finish(/* Some app.* functions */)]
/// #[cleanup(/* Some app.* functions */)]
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum PluginStage {
    Build = 0,
    Ready,
    Finish,
    Cleanup,
}

impl TryFrom<&Ident> for PluginStage {
    type Error = syn::Error;

    fn try_from(value: &Ident) -> Result<Self, Self::Error> {
        PluginStage::try_from(value.to_string().as_str())
    }
}

impl TryFrom<&Attribute> for PluginStage {
    type Error = syn::Error;

    fn try_from(value: &Attribute) -> Result<Self, Self::Error> {
        PluginStage::try_from(value.path().require_ident()?)
    }
}

impl TryFrom<&str> for PluginStage {
    type Error = syn::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "build" => Ok(PluginStage::Build),
            "ready" => Ok(PluginStage::Ready),
            "finish" => Ok(PluginStage::Finish),
            "cleanup" => Ok(PluginStage::Cleanup),
            other => Err(Error::new(
                Span::call_site(),
                format!("Unknown plugin stage `{other}`"),
            )),
        }
    }
}

impl TryFrom<usize> for PluginStage {
    type Error = &'static str;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PluginStage::Build),
            1 => Ok(PluginStage::Ready),
            2 => Ok(PluginStage::Finish),
            3 => Ok(PluginStage::Cleanup),
            _ => Err("PluginStage index out of bounds"),
        }
    }
}

impl Into<&'static str> for &PluginStage {
    fn into(self) -> &'static str {
        match self {
            PluginStage::Build => "build",
            PluginStage::Ready => "ready",
            PluginStage::Finish => "finish",
            PluginStage::Cleanup => "cleanup",
        }
    }
}

impl ToTokens for PluginStage {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PluginStage::Build => tokens.extend(quote!(build)),
            PluginStage::Ready => tokens.extend(quote!(ready)),
            PluginStage::Finish => tokens.extend(quote!(finish)),
            PluginStage::Cleanup => tokens.extend(quote!(cleanup)),
        }
    }
}

fn parse_stage_ops(input: ParseStream) -> syn::Result<PluginStageOps> {
    let mut ops = Vec::new();

    while !input.is_empty() {
        match input.parse::<Meta>()? {
            Meta::Path(path) => {
                // Just a simple `app.function()`
                ops.push(syn::parse2(quote!(#path ()))?);
            }
            Meta::List(list) => {
                // Call with arguments `app.function(arg1, arg2)`
                let path = list.path;
                let args: Punctuated<Expr, syn::token::Comma> =
                    Punctuated::parse_terminated.parse2(list.tokens)?; // What do you mean I need to call `parse2` on the fucking parse function
                ops.push(syn::parse2(quote!( #path (#args) ))?);
            }
            Meta::NameValue(name_value) => {
                // Call with a single argument `app.name(value)`
                let name = name_value.path;
                let value = name_value.value;
                ops.push(syn::parse2(quote!( #name (#value)))?);
            }
        }
        if !input.is_empty() {
            input.parse::<syn::token::Comma>()?;
        }
    }

    Ok(ops)
}

fn parse_plugin_stage_data(attrs: &mut Vec<Attribute>) -> syn::Result<PluginStageData> {
    let mut plugin_stages: PluginStageData = Default::default();

    let mut removes = HashSet::new();

    for (meta, stage) in attrs.iter().enumerate().filter_map(|(pos, attr)| {
        if let Ok(stage) = PluginStage::try_from(attr) {
            removes.insert(pos);
            return Some((attr.meta.clone(), stage));
        }
        None
    }) {
        // TODO: Handle #[ready] completely differently
        if stage == PluginStage::Ready {
            return Err(Error::new_spanned(meta, "#[ready] attributes are currently unsupported. You should define them in an annotated `impl Plugin` block."));
        }
        if plugin_stages[stage as usize].is_some() {
            return Err(Error::new_spanned(
                meta,
                format!("Multiple declarations of `{}`", Into::<&str>::into(&stage)),
            ));
        }
        match &meta {
            Meta::List(list) => {
                // #[build(...)]
                if stage == PluginStage::Ready {
                    return Err(Error::new_spanned(
                        meta,
                        "`ready` only supports name-value style (`#[ready = func]`",
                    ));
                }
                plugin_stages[stage as usize] = Some(list.parse_args_with(parse_stage_ops)?);
            }
            Meta::NameValue(name_value) => {
                // #[build = ...]
                if let Ok(func) = syn::parse2(name_value.value.to_token_stream()) {
                    plugin_stages[stage as usize] = Some(vec![func]);
                } else {
                    let value = &name_value.value;
                    // Assume it's just missing the parenthesis and try to convert it
                    let func = syn::parse2(quote!( #value () ))?;
                    plugin_stages[stage as usize] = Some(vec![func])
                }
            }
            Meta::Path(_) => {
                // #[build], invalid
                return Err(Error::new_spanned(
                    meta,
                    "Expected a name-value attribute or list of attributes, got path",
                ));
            }
        }
    }

    *attrs = attrs
        .into_iter()
        .enumerate()
        .filter_map(|(i, attr)| {
            if removes.contains(&i) {
                None
            } else {
                Some(attr.clone())
            }
        })
        .collect();

    Ok(plugin_stages)
}

impl Parse for ButlerPluginInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        match input.parse::<Item>()? {
            Item::Struct(mut plugin_struct) => {
                let stages = parse_plugin_stage_data(&mut plugin_struct.attrs)?;
                Ok(Self::Struct(plugin_struct, stages))
            }
            Item::Impl(mut plugin_impl) => {
                let stages = parse_plugin_stage_data(&mut plugin_impl.attrs)?;
                Ok(Self::Impl(plugin_impl, stages))
            }
            _ => Err(Error::new(
                input.span(),
                "#[butler_plugin] can only be used for `struct` or `impl Plugin`",
            )),
        }
    }
}

fn generate_plugin_stage_stmts(
    stage: PluginStage,
    app_ident: &Ident,
    ops: PluginStageOps,
) -> TokenStream {
    if stage == PluginStage::Ready {
        let stmt_iter = ops.into_iter().map(|op| quote!(#app_ident.#op));
        quote! {{
            #(#stmt_iter)*
        }}
    } else {
        let build_stmt = if stage == PluginStage::Build {
            Some(quote! {
                <Self as ::bevy_butler::__internal::ButlerPlugin>::register_butler_plugins(#app_ident);
            })
        } else {
            None
        };

        let stmt_iter = ops.into_iter().map(|op| quote!(#app_ident.#op;));
        quote! {{
            #build_stmt

            #(#stmt_iter)*
        }}
    }
}

fn generate_plugin_stage(stage: PluginStage, ops: PluginStageOps) -> TokenStream {
    let app_arg = if stage == PluginStage::Ready {
        quote!(app: &::bevy_butler::__internal::bevy_app::App)
    } else {
        quote!(app: &mut ::bevy_butler::__internal::bevy_app::App)
    };

    let stage_block = generate_plugin_stage_stmts(stage, &syn::parse2(quote!(app)).unwrap(), ops);

    quote! {
        fn #stage(&self, #app_arg) {
            #stage_block
        }
    }
}

fn register_plugin_block(plugin: &Type) -> Result<TokenStream, TokenStream> {
    let ident = &plugin
        .to_token_stream()
        .to_string()
        .replace("::", "_")
        .replace("<", "_")
        .replace(">", "_");
    let marker_ident = format_ident!("{}ButlerInternalPluginMarker", ident);
    Ok(quote! {
        pub struct #marker_ident {
            pub(crate) _marker: (),
        }

        impl ::bevy_butler::__internal::ButlerPlugin for #plugin {
            type Marker = #marker_ident;

            fn _marker() -> Self::Marker {
                #marker_ident { _marker: () }
            }
        }
    })
}

pub(crate) fn struct_impl(
    plugin: ItemStruct,
    mut args: PluginStageData,
) -> Result<TokenStream, TokenStream> {
    args[PluginStage::Build as usize].get_or_insert(vec![]); // `build` is a required stage
                                                             // For a struct, generate an `impl Plugin` statement below the struct declaration
    let stage_iter = args.into_iter().enumerate().map(|(stage, ops)| {
        let stage = PluginStage::try_from(stage).unwrap();
        ops.map(|ops| generate_plugin_stage(stage, ops))
    });

    let ident = &plugin.ident;

    let register_block = register_plugin_block(
        &syn::parse2(ident.to_token_stream()).map_err(|e| e.to_compile_error())?,
    )?;

    Ok(quote! {
        #plugin

        impl ::bevy_butler::__internal::bevy_app::Plugin for #ident {
            #(#stage_iter)*
        }

        #register_block
    })
}

pub(crate) fn impl_impl(
    mut plugin: ItemImpl,
    mut args: PluginStageData,
) -> Result<TokenStream, TokenStream> {
    args[PluginStage::Build as usize].get_or_insert(vec![]); // Inject a dummy `fn build` to generate the plugin registration method

    for item in plugin.items.iter_mut() {
        if let ImplItem::Fn(item) = item {
            if let Ok(stage) = PluginStage::try_from(&item.sig.ident) {
                if let Some(ops) = args[stage as usize].take() {
                    let app_ident;
                    // Why does this fucking suck so much
                    if let FnArg::Typed(pat) = &item.sig.inputs[1] {
                        if let Pat::Ident(arg) = &*pat.pat {
                            app_ident = &arg.ident;
                        } else {
                            panic!("Argument isnt an identifier?");
                        }
                    } else {
                        panic!("&self in the second argument???");
                    }

                    // Inject the setup into the existing stage
                    let stage_block = generate_plugin_stage_stmts(stage, app_ident, ops);

                    item.block.stmts.insert(
                        0,
                        syn::parse2(stage_block).map_err(|e| e.to_compile_error())?,
                    );
                }
            }
        }
    }

    // Insert any blocks that werent user-defined, but had attributes
    for (stage, ops) in args.into_iter().enumerate() {
        let stage = PluginStage::try_from(stage).unwrap();
        if let Some(ops) = ops {
            // Insert a new function block
            let stage = generate_plugin_stage(stage, ops);
            plugin
                .items
                .push(syn::parse2(stage).map_err(|e| e.to_compile_error())?);
        }
    }

    let register_block = register_plugin_block(&*plugin.self_ty)?;

    Ok(quote! {
        #plugin

        #register_block
    })
}

pub(crate) fn macro_impl(input: ButlerPluginInput) -> Result<TokenStream, TokenStream> {
    match input {
        ButlerPluginInput::Struct(plugin, args) => struct_impl(plugin, args),
        ButlerPluginInput::Impl(plugin, args) => impl_impl(plugin, args),
    }
}
