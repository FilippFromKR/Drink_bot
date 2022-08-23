use serde_json::Error;
use teloxide::dispatching::dialogue::ErasedStorage;
use teloxide::RequestError;
use url::ParseError;

use crate::telegramm::state::State;

#[derive(Debug)]
pub enum ErrorType {
    TELEGRAMM,
    DB,
    PARSE,
    USER,
    SERVICE,
    DATABASE,
    FILE,
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
            ty: ErrorType::PARSE,
        }
    }
}

impl From<reqwest::Error> for ErrorHandler {
    fn from(error: reqwest::Error) -> Self {
        Self {
            msg: error.to_string(),
            ty: ErrorType::SERVICE,
        }
    }
}


impl From<RequestError> for ErrorHandler {
    fn from(err: RequestError) -> Self {
        ErrorHandler {
            msg: err.to_string(),
            ty: ErrorType::TELEGRAMM,
        }
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for ErrorHandler {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        ErrorHandler {
            msg: err.to_string(),
            ty: ErrorType::TELEGRAMM,
        }
    }
}

impl From<url::ParseError> for ErrorHandler {
    fn from(err: ParseError) -> Self {
        Self {
            msg: err.to_string(),
            ty: ErrorType::PARSE,
        }
    }
}
