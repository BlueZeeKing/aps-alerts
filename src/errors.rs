use std::{env, error, fmt::Display, io};

#[derive(Debug)]
pub enum Error {
    NetworkError(reqwest::Error),
    RequestError(RequestError),
    FileError(io::Error),
    JsonError(serde_json::Error),
    ConfigVarError(env::VarError),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NetworkError(err) => write!(f, "Network error: {}", err),
            Error::RequestError(err) => write!(f, "Request error: {}", err),
            Error::FileError(err) => write!(f, "File error: {}", err),
            Error::JsonError(err) => write!(f, "Json parse error: {}", err),
            Error::ConfigVarError(err) => write!(f, "Config error: {}", err),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::NetworkError(value)
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::FileError(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::JsonError(value)
    }
}

impl From<env::VarError> for Error {
    fn from(value: env::VarError) -> Self {
        Error::ConfigVarError(value)
    }
}

#[derive(Debug)]
pub struct RequestError {
    pub code: reqwest::StatusCode,
    pub url: String,
    pub msg: String,
}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Request to {:?} failed with code {}: {}",
            self.url, self.code, self.msg
        )
    }
}

impl error::Error for RequestError {}
