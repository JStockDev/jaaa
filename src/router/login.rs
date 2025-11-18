use std::sync::Arc;

use argon2::{PasswordHash, PasswordVerifier, password_hash::Error};
use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;

use crate::{
    RouterState,
    db::{prelude::Users, users},
    error::CodeError,
    jwt::JWT,
};

pub async fn route(
    State(state): State<Arc<RouterState>>,
    mut jar: CookieJar,
    Json(user): Json<UserCredentials>,
) -> Result<Response, CodeError> {
    let search_user = Users::find()
        .filter(users::Column::Email.eq(&user.email))
        .one(&state.db)
        .await?;

    let user_db = match search_user {
        Some(user) => user,
        None => return Err(CodeError::NotFound),
    };

    if let Err(error) = state.hashing_algo.verify_password(
        user.password.as_bytes(),
        &PasswordHash::new(&user_db.password)?,
    ) {
        if error == Error::Password {
            return Err(CodeError::Unauthorised);
        } else {
            return Err(error.into());
        }
    }

    let user = user_db;
    let jwt = JWT::now(user.id);

    if let Some(cookie) = jar.clone().get("jaaa_access") {
        jar = jar.remove(cookie.to_owned())
    }

    let mut jwt_cookie = Cookie::new("jaaa_access", jwt.encode_and_sign(&state.jwt_key));
    jwt_cookie.set_http_only(true);
    jwt_cookie.set_secure(true);
    jwt_cookie.set_expires(jwt.get_expiry());
    jwt_cookie.set_same_site(SameSite::Strict);

    let final_jar = jar.add(jwt_cookie);
    Ok(final_jar.into_response())
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserCredentials {
    email: String,
    password: String,
}
