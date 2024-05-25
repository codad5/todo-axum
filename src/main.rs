use axum::{
    routing::get,
    Router, Json
};

use std::net::SocketAddr;

#[tokio::main]
async fn main( ) {
    let app = Router::new()
        .route("/", get(hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    println!("Listening on {}", addr);
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