use std::convert::From;
use thiserror::Error;

/// Possible Errors are summed up as `RequestError` each variant describes a possible error that could occur at different stages.
#[derive(Debug, Error)]
pub enum RequestError {
    #[error("failed to connect, because of {}", reason)]
    ConnectionError { reason: String },
    #[error("Deserialization failed to pass, the request did not deliver the structure expected.")]
    SerdeError { error: serde_urlencoded::ser::Error },

    #[error("Deserialization failed to pass, the request did not deliver the structure expected.")]
    SerdeJsonError { error: serde_json::error::Error },

    #[error("Parsing of Url has failed, given URI did not conform with standard.")]
    ParserError { error: url::ParseError },
}

impl From<reqwest::Error> for RequestError {
    fn from(_error: reqwest::Error) -> Self {
        Self::ConnectionError {
            reason: "Request failed to go through, this error happened in `reqwest`".to_string(),
        }
    }
}

impl From<serde_json::error::Error> for RequestError {
    fn from(error: serde_json::error::Error) -> Self {
        Self::SerdeJsonError { error }
    }
}

impl From<serde_urlencoded::ser::Error> for RequestError {
    fn from(error: serde_urlencoded::ser::Error) -> Self {
        Self::SerdeError { error }
    }
}

impl From<url::ParseError> for RequestError {
    fn from(error: url::ParseError) -> Self {
        Self::ParserError { error }
    }
}
