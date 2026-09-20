#[derive(Debug)]
pub struct Error(ErrorKind);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<E> From<E> for Error
where ErrorKind: From<E>
{
    fn from(value: E) -> Self {
        Self(value.into())
    }
}

#[derive(Debug)]
enum ErrorKind {
    JsonError(serde_json::Error),
    Json5Error(json5::Error),
    GJson5Error(serde_json5::Error),
    HJsonError(serde_hjson::Error),
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorKind::JsonError(error) => write!(f, "{error}"),
            ErrorKind::Json5Error(error) => write!(f, "{error}"),
            ErrorKind::HJsonError(error) => write!(f, "{error}"),
            ErrorKind::GJson5Error(error) => write!(f, "{error}"),
        }
    }
}

impl From<serde_json::Error> for ErrorKind {
    fn from(v: serde_json::Error) -> Self {
        Self::JsonError(v)
    }
}

impl From<json5::Error> for ErrorKind {
    fn from(v: json5::Error) -> Self {
        Self::Json5Error(v)
    }
}

impl From<serde_hjson::Error> for ErrorKind {
    fn from(v: serde_hjson::Error) -> Self {
        Self::HJsonError(v)
    }
}

impl From<serde_json5::Error> for ErrorKind {
    fn from(v: serde_json5::Error) -> Self {
        Self::GJson5Error(v)
    }
}
