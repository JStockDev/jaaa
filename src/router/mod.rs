use std::sync::Arc;
use argon2::Argon2;
use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;

mod health;
mod register;
mod login;
mod logout;
mod authenticate;
mod authorise;
mod whoami;

pub(super) fn build_router(router_state: RouterState) -> Router {
    Router::new()
        .route("/health", get(health::route))
        .route("/register", post(register::route))
        .route("/login", post(login::route))
        .route("/whoami", get(whoami::route))
        .route("/logout", get(logout::route))
        .with_state(Arc::new(router_state))
}

pub(super) struct RouterState {
    pub db: DatabaseConnection,
    pub hashing_algo: Argon2<'static>,
    pub jwt_key: Vec<u8>
}