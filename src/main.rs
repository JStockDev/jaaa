mod error;
mod db;
mod router;
mod jwt;

use migration::{Migrator, MigratorTrait};
use router::RouterState;

use std::env;
use argon2::{Algorithm, Argon2, Params, Version};
use sea_orm::Database;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    if cfg!(debug_assertions) {
        dotenv::dotenv().unwrap();
    }

    let db = Database::connect("postgres://auth:db_password@localhost:5432/auth_db")
        .await
        .unwrap();
    Migrator::up(&db, None).await.unwrap();

    let socket = TcpListener::bind("0.0.0.0:3000").await.unwrap();

    let hash_secret = env::var("HASH_SECRET").unwrap().into_bytes();
    let jwt_secret = env::var("JWT_SECRET").unwrap().into_bytes();

    let state = RouterState {
        db: db,
        hashing_algo: Argon2::new_with_secret(
            //Requires to have a static lifetime
            Box::leak(Box::new(hash_secret)),
            Algorithm::Argon2id,
            Version::default(),
            Params::default(),
        ).unwrap(),
        jwt_key: jwt_secret
    };
    let router = router::build_router(state);

    axum::serve(socket, router).await.unwrap();
}
