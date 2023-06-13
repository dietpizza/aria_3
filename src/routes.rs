use crate::download::Download;
use tokio::spawn;

pub async fn ping_handler() -> &'static str {
    spawn(async { run_lol().await });
    "<h1>Hello from the root path!</h1>"
}

async fn run_lol() {
    // println!("Running lol");
    let url = "http://localhost:3000/sfsymbols.json";
    let mut dl = Download::new(url, "file.bin");
    let _ = dl.connect().await;
    println!("{:?}", dl);
}
