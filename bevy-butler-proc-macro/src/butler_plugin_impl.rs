//! This file enables #[butler_plugin] to be used in two ways
//!
//! 1. Attaching it to a struct definition will generate an `impl Plugin` for it
//!
//! 2. Attaching it to an `impl Plugin` definition will add a hook at the beginning
//!    of the `build` function, or create the `build` function if one isn't present.

use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{
    parse::{Parse, ParseStream, Parser}, punctuated::Punctuated, spanned::Spanned, Attribute, Error, Expr, FnArg, Ident, ImplItem, Item, ItemImpl, ItemStruct, Meta, Pat, Path, Token, Type
};

pub(crate) type PluginStageOps = Vec<Expr>;
#[derive(Debug)]
pub(crate) struct PluginStageData(pub [Option<PluginStageOps>; 3], pub Span);

#[derive(Debug)]
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

impl ButlerPluginInput {
    pub fn stage_data(&mut self) -> &mut PluginStageData {
        match self {
            ButlerPluginInput::Impl(_, data) => data,
            ButlerPluginInput::Struct(_, data) => data,
        }
    }
}

fn parse_stage_ops(input: ParseStream) -> syn::Result<PluginStageOps> {
    let mut ops = Vec::new();
    let metas: Punctuated<Meta, Token![,]> = Punctuated::parse_terminated(input)?;

    for meta in metas {
        match meta {
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
    }

    Ok(ops)
}

/// Parse a single stage invocation, i.e. `#[build(...)]`
/// Return None if it's not a PluginStage
/// Return Some if it's a valid PluginStage path and successfully parsed
/// Return Err if it's a valid PluginStage path, but failed to parse
pub fn parse_stage(meta: &Meta) -> syn::Result<Option<(PluginStage, PluginStageOps)>> {
    let stage = PluginStage::try_from(meta.path());
    if stage.is_err() {
        // This isn't a PluginStage attr
        return Ok(None);
    }
    let stage = stage.unwrap();

    match meta {
        Meta::Path(path) => return Err(Error::new(path.span(), "Expected list or name-value, got path")),
        Meta::List(list) => return Ok(Some((stage, list.parse_args_with(parse_stage_ops)?))),
        Meta::NameValue(name_value) => Ok(Some((stage, vec![name_value.value.clone()]))),
    }
}

impl PluginStageData {
    pub fn merge(&mut self, other: PluginStageData) -> syn::Result<()> {
        for (stage, ops) in other.0.into_iter().enumerate().filter_map(|(stage, ops)| ops.map(|ops| (PluginStage::try_from(stage).unwrap(), ops))) {
            self.insert(stage, ops)?;
        }

        Ok(())
    }

    pub fn insert(&mut self, stage: PluginStage, ops: PluginStageOps) -> syn::Result<()> {
        if self.0[stage as usize].is_some() {
            return Err(Error::new(self.1, format!("Plugin stage `{stage}` declared multiple times")));
        }

        self.0[stage as usize] = Some(ops);
        Ok(())
    }

    /// Parse args from #[butler_plugin(...)]
    pub fn parse_as_list(list: ParseStream) -> syn::Result<PluginStageData> {
        let list: Punctuated<Meta, Token![,]> = Punctuated::parse_terminated(list)?;
        let mut stage_data = PluginStageData(
            Default::default(),
            list.span()
        );

        for meta in list {
            if let Some((stage, ops)) = parse_stage(&meta)? {
                stage_data.insert(stage, ops)?;
            }
        }
        
        Ok(stage_data)
    }

    /// Parse args from
    /// ```ignore
    /// #[butler_plugin]
    /// #[build = ...]
    /// #[cleanup = ...]
    /// ```
    pub fn parse_from_attrs(attrs: &mut Vec<Attribute>) -> syn::Result<PluginStageData> {
        let mut removes = HashSet::new();
        let mut stage_data = PluginStageData(
            Default::default(),
            Span::call_site(),
        );

        for (pos, attr) in attrs.iter().enumerate() {
            if let Some((stage, ops)) = parse_stage(&attr.meta)? {
                stage_data.insert(stage, ops)?;
                removes.insert(pos);
            }
        }

        removes.into_iter().sorted_unstable().rev().for_each(|rem| { attrs.remove(rem); });
        Ok(stage_data)
    }
}

/// ```ignore
/// #[butler_plugin]
/// #[build(/* Some app.* functions */)]
/// #[finish(/* Some app.* functions */)]
/// #[cleanup(/* Some app.* functions */)]
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum PluginStage {
    Build = 0,
    Finish,
    Cleanup,
}

impl Display for PluginStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into())
    }
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
            1 => Ok(PluginStage::Finish),
            2 => Ok(PluginStage::Cleanup),
            _ => Err("PluginStage index out of bounds"),
        }
    }
}

impl TryFrom<&Path> for PluginStage {
    type Error = syn::Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        value.get_ident()
            .and_then(|i| PluginStage::try_from(i).ok())
            .ok_or_else(|| Error::new(value.span(), format!("Unknown plugin stage \"{}\"", value.to_token_stream().to_string())))
    }
}

impl Into<&'static str> for &PluginStage {
    fn into(self) -> &'static str {
        match self {
            PluginStage::Build => "build",
            PluginStage::Finish => "finish",
            PluginStage::Cleanup => "cleanup",
        }
    }
}

impl ToTokens for PluginStage {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PluginStage::Build => tokens.extend(quote!(build)),
            PluginStage::Finish => tokens.extend(quote!(finish)),
            PluginStage::Cleanup => tokens.extend(quote!(cleanup)),
        }
    }
}

impl Parse for ButlerPluginInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        match input.parse::<Item>()? {
            Item::Struct(mut plugin_struct) => {
                let stages = PluginStageData::parse_from_attrs(&mut plugin_struct.attrs)?;
                Ok(Self::Struct(plugin_struct, stages))
            }
            Item::Impl(mut plugin_impl) => {
                let stages = PluginStageData::parse_from_attrs(&mut plugin_impl.attrs)?;
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

fn generate_plugin_stage(stage: PluginStage, ops: PluginStageOps) -> TokenStream {
    let stage_block = generate_plugin_stage_stmts(stage, &syn::parse2(quote!(app)).unwrap(), ops);

    quote! {
        fn #stage(&self, app: &mut ::bevy_butler::__internal::bevy_app::App) {
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
    let _ = args.insert(PluginStage::Build, vec![]); // Inject a dummy `fn build` to generate the plugin registration method

    let stage_iter = args.0.into_iter().enumerate().map(|(stage, ops)| {
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
    let _ = args.insert(PluginStage::Build, vec![]); // Inject a dummy `fn build` to generate the plugin registration method

    for item in plugin.items.iter_mut() {
        if let ImplItem::Fn(item) = item {
            if let Ok(stage) = PluginStage::try_from(&item.sig.ident) {
                if let Some(ops) = args.0[stage as usize].take() {
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
    for (stage, ops) in args.0.into_iter().enumerate() {
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
