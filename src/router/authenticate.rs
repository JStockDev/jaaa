use std::sync::Arc;

use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::extract::CookieJar;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use time::OffsetDateTime;

use crate::{
    RouterState,
    db::users::{self, Entity, Model as UserModel},
    error::CodeError,
    jwt::JWT,
};

pub async fn logic(state: Arc<RouterState>, jar: CookieJar) -> Result<UserModel, CodeError> {
    let cookie = if let Some(c) = jar.get("jaaa_access") {
        c
    } else {
        return Err(CodeError::Unauthorised);
    };

    let (jwt, signature) = JWT::decode(cookie.value().to_string())?;
    if jwt.verify_signature(signature, &state.jwt_key) == false {
        return Err(CodeError::Unauthorised);
    }

    if OffsetDateTime::now_utc() >= jwt.get_expiry() {
        return Err(CodeError::Unauthorised);
    }

    Entity::find()
        .filter(users::Column::Id.eq(jwt.get_user_id()))
        .one(&state.db)
        .await?
        .ok_or_else(|| CodeError::NotFound)
}

pub struct Auth(pub users::Model);

impl<S> FromRequestParts<S> for Auth
where
    Arc<RouterState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = CodeError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let router_state = Arc::from_ref(state);
        let jar = CookieJar::from_request_parts(parts, state).await?;

        Ok(Auth(logic(router_state, jar).await?))
    }
}
