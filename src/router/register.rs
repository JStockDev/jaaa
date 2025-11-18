use std::sync::Arc;

use argon2::{
    PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State, http::StatusCode};
use sea_orm::{ActiveValue::Set, ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    RouterState,
    db::{prelude::Users, users},
    error::CodeError,
};

pub async fn route(
    State(state): State<Arc<RouterState>>,
    Json(user): Json<UserCredentials>,
) -> Result<StatusCode, CodeError<>> {
    if Users::find()
        .filter(users::Column::Email.eq(&user.email))
        .one(&state.db)
        .await?
        .is_some()
    {
        return Err(CodeError::Conflict);
    }

    let salt = SaltString::generate(&mut OsRng);
    let hash = state.hashing_algo.hash_password(user.password.as_bytes(), &salt)?;

    let user_entry = users::ActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set(user.email),
        user_name: Set(user.username),
        password: Set(hash.to_string()),
        created_at: Set(time::OffsetDateTime::now_utc()),
    };

    Users::insert(user_entry).exec(&state.db).await?;
    Ok(StatusCode::CREATED)
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserCredentials {
    username: String,
    email: String,
    password: String,
}
