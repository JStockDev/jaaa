use std::sync::Arc;

use axum_extra::extract::CookieJar;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

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

    Entity::find()
        .filter(users::Column::Id.eq(jwt.get_user_id()))
        .one(&state.db)
        .await?
        .ok_or_else(|| CodeError::NotFound)
}
