use std::fmt::Display;

use syn::{Error, Ident, Path};

#[derive(Clone, Copy)]
#[repr(u8)]
pub(crate) enum PluginStage {
    Build = 0,
    Finish,
    Cleanup
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
