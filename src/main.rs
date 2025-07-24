use axum::{routing::get, Extension, Router};
use dotenv::dotenv;
use sea_orm::Database;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;

pub mod controllers;
pub mod entity;
pub mod models;
pub mod routes;
pub mod services;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_conn = match Database::connect(&database_url).await {
        Ok(conn) => conn,
        Err(err) => {
            println!("Failed to connect to PostgreSQL: {}", err);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/", get(root))
        .merge(routes::users::user_routes())
        .layer(Extension(Arc::new(db_conn)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Health Check - Healthy!"
}
