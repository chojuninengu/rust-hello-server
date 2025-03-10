use axum::{routing::get, Router};
use std::net::SocketAddr;
use hyper::Server; // Import Server from hyper

#[tokio::main]
async fn main() {
    // Define the router with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // Define the address to bind to
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // Start the server
    println!("Listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
