mod download;
mod routes;
mod scheduler;

use scheduler::_download_scheduler;

use {
    axum::{routing::get, Router},
    routes::ping_handler,
    std::net::SocketAddr,
};

#[tokio::main]
async fn main() {
    // let app = Router::new().route("/", get(ping_handler));
    // let socket_addr = SocketAddr::from(([0, 0, 0, 0], 5000));

    // println!("Starting server...");
    // axum::Server::bind(&socket_addr)
    // .serve(app.into_make_service())
    // .await
    // .unwrap();
    //
    let _ = tokio::spawn(async {
        _download_scheduler().await;
    })
    .await;
}
