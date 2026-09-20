pub use serde_json::{
    Value,
    Map,
    Number,
};

pub use error::Error;

mod error;
mod supp;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum Spec {
    #[default]
    Json,
    Json5,
    HJson,
}

impl Spec {
    pub fn all() -> &'static [Self] {
        &[
            Self::Json,
            Self::Json5,
            Self::HJson,
        ]
    }
}

pub trait SpecDe: Sized {
    fn from_specjson(spec: Spec, s: &str) -> Result<Self, Error>;
}
pub trait SpecSer: Sized {
    fn to_specjson(&self, spec: Spec) -> Result<String, Error>;
    fn to_specjson_prettify(&self, spec: Spec) -> Result<String, Error>;
}

impl<T: serde::de::DeserializeOwned> SpecDe for T {
    fn from_specjson(spec: Spec, s: &str) -> Result<Self, Error> {
        Ok(match spec {
            Spec::Json => serde_json::from_str(s)?,
            Spec::Json5 => json5::from_str(s)?,
            Spec::HJson => serde_hjson::from_str(s)?,
        })
    }
}

impl<T: serde::Serialize> SpecSer for T {
    fn to_specjson(&self, spec: Spec) -> Result<String, Error> {
        Ok(match spec {
            Spec::Json => serde_json::to_string(self)?,
            // FIXME: 当 json5 库支持 compact 输出后, 就放弃 serde_json5
            Spec::Json5 => serde_json5::to_string(self)?,
            Spec::HJson => {
                let mut writer = Vec::with_capacity(128);
                let mut ser = serde_hjson::Serializer::with_formatter(
                    &mut writer,
                    supp::HjsonCompactFormatter,
                );
                serde::Serialize::serialize(self, &mut ser)?;
                String::from_utf8(writer).map_err(serde_hjson::Error::from)?
            },
        })
    }

    fn to_specjson_prettify(&self, spec: Spec) -> Result<String, Error> {
        Ok(match spec {
            Spec::Json => serde_json::to_string_pretty(self)?,
            Spec::Json5 => json5::to_string(self)?,
            Spec::HJson => serde_hjson::to_string(self)?,
        })
    }
}

#[cfg(test)]
mod tests;
