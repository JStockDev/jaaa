use std::sync::Arc;

use axum::{extract::State, response::{IntoResponse, Response}};
use axum_extra::extract::CookieJar;
use crate::{RouterState, error::CodeError, router::authenticate};

pub async fn route(
    State(state): State<Arc<RouterState>>,
    jar: CookieJar,
) -> Result<Response, CodeError> {
    authenticate::logic(state, jar.clone()).await?;
    let cookie = jar.get("jaaa_access").unwrap();
    Ok(jar.clone().remove(cookie.clone()).into_response())
}

