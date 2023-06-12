use crate::download::Download;
use {axum::response::Html, tokio::spawn};

pub async fn ping_handler() -> Html<&'static str> {
    spawn(async { run_lol().await });
    Html("<h1>Hello from the root path!</h1>")
}

async fn run_lol() {
    println!("Running lol");
    let url = "https://speed.hetzner.de/1GB.bin";
    let mut dl = Download::new(url, "file.bin");
    let _ = dl.connect().await;

    println!("Download {:?}", dl);
}
