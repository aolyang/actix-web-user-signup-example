use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::{Serialize, Serializer};
use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct AppErrorCode(i32);

#[derive(Debug, Serialize)]
pub struct AppError {
    pub code: AppErrorCode,
    pub message: String,
}

impl Serialize for AppErrorCode {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.0)
    }
}

impl AppErrorCode {
    pub fn message(self, message: String) -> AppError {
        AppError {
            message,
            code: self,
        }
    }

    pub fn default(self) -> AppError {
        let message = match self {
            AppError::INVALID_INPUT => "Invalid input.",
            AppError::NOT_AUTHORIZED => "Not authorized.",
            _ => "An unexpected error has occurred.",
        };
        AppError {
            code: self,
            message: message.to_string(),
        }
    }
}

impl AppError {
    /// (1000..2000) error from request input  
    pub const INVALID_INPUT: AppErrorCode = AppErrorCode(1000);
    /// (1100..1200) users
    pub const INVALID_USER_EMAIL: AppErrorCode = AppErrorCode(1101);
    pub const INVALID_USER_NAME: AppErrorCode = AppErrorCode(1102);
    /// (2000..3000) permissions
    // pub const PERMISSION_ERROR: AppErrorCode = AppErrorCode(2000);
    pub const NOT_AUTHORIZED: AppErrorCode = AppErrorCode(2001);
    /// (4000..5000) database
    pub const DATABASE_ERROR: AppErrorCode = AppErrorCode(4000);
    /// (5000..6000) unknown server error
    // pub const SERVER_ERROR: AppErrorCode = AppErrorCode(5000);
    /// (6000..7000) actix web error
    pub const ACTIX_ERROR: AppErrorCode = AppErrorCode(6000);
    // pub const ACTIX_SERDE_ERROR: AppErrorCode = AppErrorCode(6001);
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<AppErrorCode> for AppError {
    fn from(err: AppErrorCode) -> Self {
        err.default()
    }
}

impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.code {
            AppError::INVALID_INPUT => StatusCode::BAD_REQUEST,
            AppError::NOT_AUTHORIZED => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(self)
    }
}

impl From<actix_web::error::Error> for AppError {
    fn from(err: actix_web::error::Error) -> Self {
        AppError {
            code: AppError::ACTIX_ERROR,
            message: err.to_string(),
        }
    }
}

impl From<sqlx::error::Error> for AppError {
    fn from(err: sqlx::error::Error) -> Self {
        AppError {
            code: AppError::DATABASE_ERROR,
            message: err.to_string(),
        }
    }
}
