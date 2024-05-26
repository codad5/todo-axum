use axum::{
    routing::get,
    Router, Json
};

use std::net::SocketAddr;

#[tokio::main]
async fn main( ) {
    println!("Starting server app : {}", std::env::var("CARGO_PKG_NAME").unwrap());
    let app = Router::new()
        .route("/", get(hello));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}


#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn hello() -> Json<Message> {
    Json(Message {
        message: "Hello, World!".to_string(),
    })
}