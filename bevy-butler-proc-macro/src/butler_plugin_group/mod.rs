use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use structs::ButlerPluginGroupAttr;
use syn::{Error, ImplItemFn, Item};

pub(crate) mod structs;

pub(crate) fn macro_impl(attr: TokenStream1, body: TokenStream1) -> syn::Result<TokenStream2> {
    let attr: ButlerPluginGroupAttr = deluxe::parse(attr)?;
    let item: Item = syn::parse(body)?;

    let name_func: Option<ImplItemFn> = attr.name.map(|name| {
        syn::parse_quote! {
            fn name() -> String {
                #name.to_string()
            }
        }
    });

    match item {
        Item::Struct(i_struct) => {
            let ident = &i_struct.ident;

            Ok(quote! {
                #i_struct

                impl #ident {
                    pub(crate) fn _butler_plugin_group_sealed_marker() -> ::std::any::TypeId {
                        struct SealedMarker;

                        std::any::TypeId::of::<SealedMarker>()
                    }
                }

                impl ::bevy_butler::__internal::ButlerPluginGroup for #ident {}

                impl ::bevy_butler::__internal::bevy_app::PluginGroup for #ident {
                    fn build(self) -> ::bevy_butler::__internal::bevy_app::PluginGroupBuilder {
                        <Self as ::bevy_butler::__internal::ButlerPluginGroup>
                            ::register_plugins(::bevy_butler::__internal::bevy_app::PluginGroupBuilder::start::<Self>(), #ident::_butler_plugin_group_sealed_marker())
                    }

                    #name_func
                }
            })
        }
        other => Err(Error::new_spanned(other, "Expected `struct`"))
    }
}