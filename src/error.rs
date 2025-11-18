use std::error::Error;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum CodeError {
    Unauthorised,
    Conflict,
    NotFound, 
    DecodeError,
    InternalError,
}

impl<E> From<E> for CodeError
where
    E: Error + Send + Sync + 'static,
{
    fn from(_: E) -> Self {
        // Do error logging here
        Self::InternalError
    }
}

impl IntoResponse for CodeError {
    fn into_response(self) -> Response {
        match self {
            CodeError::Unauthorised => StatusCode::UNAUTHORIZED,
            CodeError::Conflict => StatusCode::CONFLICT,
            CodeError::NotFound => StatusCode::NOT_FOUND,
            CodeError::DecodeError => StatusCode::BAD_REQUEST,
            CodeError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }.into_response()
    }
}
