use thiserror::Error;
use tonic::Status;

#[derive(Clone, Debug, Error)]
pub enum AppError {
    // http status 400
    #[error("{0}")]
    BadRequest(String),

    // http status 404
    #[error("{0}")]
    NotFound(String),

    // http status 500
    #[error("{0}")]
    Internal(String),
}

impl From<AppError> for Status {
    fn from(err: AppError) -> Status {
        match err {
            AppError::BadRequest(_) => Status::invalid_argument(err.to_string()),
            AppError::NotFound(_) => Status::not_found(err.to_string()),
            AppError::Internal(_) => Status::internal(err.to_string()),
        }
    }
}
