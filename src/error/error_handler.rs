use serde_json::Error;
use teloxide::RequestError;
use url::ParseError;

#[derive(Debug)]
pub enum ErrorType {
    Telegramm,
    Unexpected,
    Parse,
    Service,
    Database,
}

#[derive(Debug)]
pub struct ErrorHandler {
    pub msg: String,
    pub ty: ErrorType,
}

impl ErrorHandler {
    pub fn is_critical(&self) -> bool {
        matches!(self.ty, ErrorType::Service | ErrorType::Database)
    }
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
