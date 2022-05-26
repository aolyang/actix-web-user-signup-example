use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum ResError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct ResInfo {
    error_message: String,
}

impl fmt::Display for ResError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl ResError {
    pub fn error_response(&self) -> String {
        match self {
            ResError::DBError(msg) => {
                println!("Database error occurred {:?}", msg);
                format!("Database Error {}", msg)
            }
            ResError::ActixError(msg) => {
                println!("Server error occurred {:?}", msg);
                "Internal server error".into()
            }
            ResError::NotFound(msg) => {
                println!("Page not found");
                msg.into()
            }
        }
    }
}

impl actix_web::error::ResponseError for ResError {
    fn status_code(&self) -> StatusCode {
        match self {
            ResError::DBError(..) | ResError::ActixError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            ResError::NotFound(..) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ResInfo {
            error_message: self.error_response(),
        })
    }
}

impl From<actix_web::error::Error> for ResError {
    fn from(err: actix_web::error::Error) -> Self {
        ResError::ActixError(err.to_string())
    }
}

impl From<sqlx::error::Error> for ResError {
    fn from(err: sqlx::error::Error) -> Self {
        ResError::DBError(err.to_string())
    }
}
