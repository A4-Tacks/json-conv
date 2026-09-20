pub use serde_json::{
    Value,
    Map,
    Number,
};

pub use error::Error;

mod error;

#[non_exhaustive]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum Spec {
    #[default]
    Json,
    Json5,
    HJson,
}

pub trait SpecConv: Sized {
    fn from_spec(spec: Spec, s: &str) -> Result<Self, Error>;

    fn to_spec(&self, spec: Spec) -> Result<String, Error>;
    fn to_spec_prettify(&self, spec: Spec) -> Result<String, Error>;
}

impl SpecConv for Value {
    fn from_spec(spec: Spec, s: &str) -> Result<Self, Error> {
        Ok(match spec {
            Spec::Json => serde_json::from_str(s)?,
            Spec::Json5 => serde_json5::from_str(s)?,
            Spec::HJson => serde_hjson::from_str(s)?,
        })
    }

    fn to_spec(&self, spec: Spec) -> Result<String, Error> {
        Ok(match spec {
            Spec::Json => serde_json::to_string(self)?,
            Spec::Json5 => serde_json5::to_string(self)?,
            Spec::HJson => serde_hjson::to_string(self)?,
        })
    }

    fn to_spec_prettify(&self, spec: Spec) -> Result<String, Error> {
        Ok(match spec {
            Spec::Json => serde_json::to_string_pretty(self)?,
            Spec::Json5 => {},
            Spec::HJson => serde_hjson::to_string(self)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    }
}
