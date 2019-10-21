use std::convert::From;
use surf::Exception;

#[derive(Debug, Fail)]
pub enum RequestError {
    #[fail(display = "failed to connect, because of {}", reason)]
    ConnectionError { reason: String },
    #[fail(
        display = "Deserialization failed to pass, the request did not deliver the structure expected."
    )]
    SerdeError { error: serde_json::error::Error },

    #[fail(display = "Parsing of Url has failed, given URI did not conform with standard.")]
    ParserError { error: url::ParseError },
}

impl From<Exception> for RequestError {
    fn from(_error: Exception) -> Self {
        Self::ConnectionError {
            reason: "Request failed to go through, this error happened in `surf`".to_string(),
        }
    }
}

impl From<serde_json::error::Error> for RequestError {
    fn from(error: serde_json::error::Error) -> Self {
        Self::SerdeError { error }
    }
}

impl From<url::ParseError> for RequestError {
    fn from(error: url::ParseError) -> Self {
        Self::ParserError { error }
    }
}
