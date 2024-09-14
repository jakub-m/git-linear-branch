use std::string::FromUtf8Error;

pub struct AppError {
    message: String,
}

impl AppError {
    pub fn new(message: &str) -> AppError {
        AppError {
            message: message.to_owned(),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError {
            message: format!("IO error: {}", value.to_string()),
        }
    }
}

impl From<FromUtf8Error> for AppError {
    fn from(value: FromUtf8Error) -> Self {
        AppError {
            message: format!("UTF8 error: {}", value.to_string()),
        }
    }
}

impl From<AppError> for String {
    fn from(value: AppError) -> Self {
        value.message
    }
}
