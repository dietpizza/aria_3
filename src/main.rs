use reqwest::Client;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use tokio::task;

async fn download_file() -> Result<bool, Box<dyn Error + Send + Sync>> {
    let url = "http://localhost:3000/sfsymbols.json";
    let file_path = "sfsymbols.bin";
    let client = Client::new();

    let mut response = client.get(url).send().await?;
    let mut file = File::create(file_path)?;
    let mut chunks = 0;
    let mut size = 0;

    while let Some(chunk) = response.chunk().await? {
        size += &chunk.len();
        chunks += 1;
        file.write_all(&chunk)?;
    }

    println!("Total chunks: {chunks}; Total downloaded size: {size} Bytes");
    Ok(true)
}

#[tokio::main]
async fn main() {
    println!("Waiting...");
    let _result = task::spawn(download_file())
        .await
        .unwrap()
        .expect("Crash!!");

    println!("Downloaded!! {:?}", _result);
}
