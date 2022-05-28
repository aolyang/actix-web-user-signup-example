use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum AppError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct ResInfo {
    error_message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl AppError {
    pub fn error_response(&self) -> String {
        match self {
            AppError::DBError(msg) => {
                println!("Database error occurred {:?}", msg);
                format!("Database Error {}", msg)
            }
            AppError::ActixError(msg) => {
                println!("Server error occurred {:?}", msg);
                "Internal server error".into()
            }
            AppError::NotFound(msg) => {
                println!("Page not found");
                msg.into()
            }
        }
    }
}

impl actix_web::error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::DBError(..) | AppError::ActixError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound(..) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ResInfo {
            error_message: self.error_response(),
        })
    }
}

impl From<actix_web::error::Error> for AppError {
    fn from(err: actix_web::error::Error) -> Self {
        AppError::ActixError(err.to_string())
    }
}

impl From<sqlx::error::Error> for AppError {
    fn from(err: sqlx::error::Error) -> Self {
        AppError::DBError(err.to_string())
    }
}
