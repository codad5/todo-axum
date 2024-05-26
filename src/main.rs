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
    
    const HOSTS : [[u8; 4]; 2] = [[127, 0, 0, 1], [0, 0, 0, 0]];
    let port : u16 = get_port();
    // try connecting to any host , the host that first connect without error will be used
    while let Some(host) = HOSTS.iter().next() {
        let addr = SocketAddr::from((*host, port));

        let listener = tokio::net::TcpListener::bind(&addr).await;
        if listener.is_ok() {
            println!("Listening on {}", addr);
            axum::serve(listener.unwrap(), app).await.unwrap();
            break;
        }
    }

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

fn get_port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000)
}