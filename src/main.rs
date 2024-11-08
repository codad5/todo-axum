
use apiresponse::{ApiResponse, ResponseStatus};
use axum::{
    extract::{Json as JsonExtract, Path, Query}, routing::{get, post}, Json, Router
};
use models::user::User;

use std::{net::SocketAddr, str};

mod models;
mod database;
mod orm;
mod apiresponse;
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
        // get all users
        .route("/users", get(get_all_users))
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



#[axum::debug_handler]
async fn hello() -> Json<ApiResponse<String>> {
    Json(ApiResponse::new(ResponseStatus::Success, "Hello", Some("World".to_string())))
}

async fn  get_user_by_id(Path(user_id) : Path<u32>) -> Json<ApiResponse<User>> {
    let user = User::get_user_by_id(user_id).await;
    match user {
        Ok(user) => Json(ApiResponse::new(ResponseStatus::Success, "User found", Some(user))),
        Err(e) => Json(ApiResponse::new(ResponseStatus::InternalServerError, &e.to_string(), None)),
    }
}

async fn create_user(JsonExtract(user) : Json<User>) -> Json<ApiResponse<User>> {
    match user.create_user().await {
        Ok(user) => Json(ApiResponse::new(ResponseStatus::Created, "User created successfully", Some(user))),
        Err(e) => Json(ApiResponse::new(ResponseStatus::InternalServerError, &e.to_string(), None)),
    }
    
}

async fn get_all_users() -> Json<ApiResponse<Vec<User>>> {
    let users = User::get_all_users().await;
    match users {
        Ok(users) => Json(ApiResponse::new(ResponseStatus::Success, "Users found", Some(users))),
        Err(e) => Json(ApiResponse::new(ResponseStatus::InternalServerError, &e.to_string(), None)),
    }
}