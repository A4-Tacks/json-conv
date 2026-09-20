#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    JsonError(serde_json::Error),
    Json5Error(serde_json5::Error),
    HJsonError(serde_hjson::Error),
}

impl From<serde_json::Error> for Error {
    fn from(v: serde_json::Error) -> Self {
        Self::JsonError(v)
    }
}

impl From<serde_json5::Error> for Error {
    fn from(v: serde_json5::Error) -> Self {
        Self::Json5Error(v)
    }
}

impl From<serde_hjson::Error> for Error {
    fn from(v: serde_hjson::Error) -> Self {
        Self::HJsonError(v)
    }
}
