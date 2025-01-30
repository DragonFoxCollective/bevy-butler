use syn::{
    parse::{Parse, ParseStream},
    Error, Token,
};

use super::PluginStageData;

pub(crate) struct ButlerPluginAttr {
    pub stages: [Option<PluginStageData>; 3],
}

impl ButlerPluginAttr {
    pub fn parse_inner(input: ParseStream) -> syn::Result<Self> {
        let mut ret = ButlerPluginAttr {
            stages: Default::default(),
        };

        for (stage, data) in input
            .parse_terminated(PluginStageData::parse, Token![,])?
            .into_iter()
            .map(|d| (d.stage, d))
        {
            if ret.stages[usize::from(stage)].is_some() {
                return Err(Error::new(
                    data.attr_span,
                    format!("Multiple declarations of \"{}\"", data.stage),
                ));
            }
            ret.stages[usize::from(stage)] = Some(data);
        }

        Ok(ret)
    }
}
