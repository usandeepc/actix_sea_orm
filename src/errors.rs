use actix_web::{error, http::StatusCode};
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Validation Failed With Error: \n {}", .serde_json::to_string(error.errors()).unwrap())]
    ValidationError { error: ValidationErrors },
    #[error("Some DB Error")]
    DatabaseError(#[from] sea_orm::DbErr),
    #[error("Other Error")]
    OtherError,
    #[error("Deserialization Error")]
    SerdeError(#[from] serde_json::Error)
}
impl error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::SerdeError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
