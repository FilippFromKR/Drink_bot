use serde_json::Error;
use teloxide::RequestError;
use url::ParseError;

#[derive(Debug)]
pub enum ErrorType {
    Telegramm,
    Db,
    Parse,
    User,
    Service,
    Database,
    File,
}

#[derive(Debug)]
pub struct ErrorHandler {
    pub msg: String,
    pub ty: ErrorType,
}

impl From<serde_json::Error> for ErrorHandler {
    fn from(error: Error) -> Self {
        Self {
            msg: error.to_string(),
            ty: ErrorType::Parse,
        }
    }
}

impl From<reqwest::Error> for ErrorHandler {
    fn from(error: reqwest::Error) -> Self {
        Self {
            msg: error.to_string(),
            ty: ErrorType::Service,
        }
    }
}

impl From<RequestError> for ErrorHandler {
    fn from(err: RequestError) -> Self {
        ErrorHandler {
            msg: err.to_string(),
            ty: ErrorType::Telegramm,
        }
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for ErrorHandler {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        ErrorHandler {
            msg: err.to_string(),
            ty: ErrorType::Telegramm,
        }
    }
}

impl From<url::ParseError> for ErrorHandler {
    fn from(err: ParseError) -> Self {
        Self {
            msg: err.to_string(),
            ty: ErrorType::Parse,
        }
    }
}
