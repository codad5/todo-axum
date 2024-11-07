
use axum::{
    extract::{Json as JsonExtract, Path, Query}, routing::{get, post}, Json, Router
};

use std::{net::SocketAddr, str};

mod models;
mod database;
mod orm;
#[cfg(test)]
mod tests;


#[tokio::main]
async fn main( ) {

    let db = database::get_pool().await.expect("Failed to connect to database");
    println!("Starting server app : {}", std::env::var("CARGO_PKG_NAME").unwrap());
    let app = Router::new()
        .route("/", get(hello))
        .route("/user/:user_id", get(get_user_by_id))
        .route("/user/create", post(create_user))
        .with_state(db);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}


#[derive(serde::Serialize)]
struct Message {
    message: String,
}


#[derive(serde::Deserialize)]
struct User {
    id: u32,
    name: String,
}

async fn hello() -> Json<Message> {
    Json(Message {
        message: "Hello, World!".to_string(),
    })
}

async fn  get_user_by_id(Path(user_id) : Path<u32>) -> Json<Message> {
    Json(Message {
        message: format!("Hello, User with id: {}", user_id),
    })
}

async fn create_user(JsonExtract(user) : Json<User>) -> Json<Message> {
    Json(Message {
        message: format!("Hello, User with id: {}", user.id),
    })
}