use std::convert::From;
use surf::Exception;


#[derive(Debug, Fail)]
pub enum RequestError {

    #[fail(display = "failed to connect to {}", url)]
    ConnectionError {
        url: String,
    }
}

impl From<Exception> for RequestError {
    fn from(error: Exception) -> Self {
        Self::ConnectionError {
            url: "foo".to_string(),
        }
    }
}
