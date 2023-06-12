mod download;
mod routes;
mod scheduler;

use {
    axum::{routing::get, Router},
    routes::ping_handler,
    std::net::SocketAddr,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(ping_handler));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    println!("Starting server...");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
