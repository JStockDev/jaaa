use std::sync::Arc;

use axum::{extract::State, response::{IntoResponse, Response}};
use axum_extra::extract::CookieJar;
use serde::Serialize;

use crate::{RouterState, error::CodeError, router::authenticate};

pub async fn route(
    State(state): State<Arc<RouterState>>,
    jar: CookieJar,
) -> Result<Response, CodeError> {
    let user = authenticate::logic(state, jar).await?;
    let data = UserData {
        username: user.user_name,
        email: user.email
    };

    Ok(serde_json::to_string(&data).unwrap().into_response())
}

#[derive(Serialize)]
struct UserData {
    username: String,
    email: String
}