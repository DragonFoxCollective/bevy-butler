use std::fmt::Display;

use quote::{ToTokens, TokenStreamExt};
use syn::{spanned::Spanned, Error, Ident, Path};

#[derive(Clone, Copy)]
#[repr(u8)]
pub(crate) enum PluginStage {
    Build,
    Finish,
    Cleanup,
}

impl From<PluginStage> for &'static str {
    fn from(value: PluginStage) -> Self {
        match value {
            PluginStage::Build => "build",
            PluginStage::Finish => "finish",
            PluginStage::Cleanup => "cleanup",
        }
    }
}

impl From<PluginStage> for Ident {
    fn from(value: PluginStage) -> Self {
        Ident::new(From::from(value), value.span())
    }
}

impl From<PluginStage> for usize {
    fn from(value: PluginStage) -> Self {
        match value {
            PluginStage::Build => 0,
            PluginStage::Cleanup => 1,
            PluginStage::Finish => 2,
        }
    }
}

impl Display for PluginStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(<Self as Into<&'static str>>::into(*self))
    }
}

impl TryFrom<&Ident> for PluginStage {
    type Error = Error;

    fn try_from(value: &Ident) -> Result<Self, Self::Error> {
        match value {
            value if value == "build" => Ok(PluginStage::Build),
            value if value == "finish" => Ok(PluginStage::Finish),
            value if value == "cleanup" => Ok(PluginStage::Cleanup),
            _ => Err(Error::new_spanned(value, format!("Unknown plugin stage \"{value}\""))),
        }
    }
}

impl TryFrom<Ident> for PluginStage {
    type Error = Error;

    fn try_from(value: Ident) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&Path> for PluginStage {
    type Error = Error;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        value.require_ident().and_then(|ident| Self::try_from(ident))
    }
}

impl TryFrom<Path> for PluginStage {
    type Error = Error;

    fn try_from(value: Path) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl ToTokens for PluginStage {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        tokens.append(Ident::from(*self));
    }
}