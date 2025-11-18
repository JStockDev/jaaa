use std::sync::Arc;
use axum::{extract::State, http::StatusCode};

use crate::{RouterState, error::CodeError};

pub async fn route(State(state): State<Arc<RouterState>>) -> Result<StatusCode, CodeError> {
    state.db.ping().await?;

    Ok(StatusCode::OK)
}