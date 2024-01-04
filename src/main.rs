//! Hello world example for Axum

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // depend on tracing for all sorts of logging
    tracing_subscriber::fmt().init();

    // build our router/application with single route
    let app = Router::new()
        .route("/", get(|| async { "Hello World!" }))
        .route("/greet/", get(|| async { "Hey! glad to have you here!" }));

    // define the TCP listener
    let listener = tokio::net::TcpListener::bind("127.0.0.1:19699")
        .await
        .unwrap();

    tracing::debug!("Listening on port: {}", listener.local_addr().unwrap());

    // info!("API server started...");

    // listen to/start the server
    axum::serve(listener, app).await.unwrap();
}
