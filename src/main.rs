use std::error::Error;
use std::fs::File;
use std::io::Write;

// use std::sync::Arc;
// use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

use reqwest::{header, Client};

const CHUNK_SIZE: u64 = 512 * 1024;

async fn download_chunk(
    url: &str,
    file_path: &str,
    range: &str,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let client = Client::new();
    let range_header = header::HeaderValue::from_str(range)?;

    let mut response = client
        .get(url)
        .header(header::RANGE, range_header)
        .send()
        .await?;
    let mut file = File::create(file_path)?;

    while let Some(chunk) = response.chunk().await? {
        sleep(Duration::from_millis(100)).await;
        file.write_all(&chunk)?;
    }

    Ok(true)
}

async fn get_chunk_vec(url: &str) -> Result<Vec<u64>, Box<dyn Error>> {
    let client = Client::new();
    let response = client.head(url).send().await?;

    let content_length: u64 = response
        .headers()
        .get("Content-Length")
        .ok_or("LOL")?
        .to_str()?
        .parse()?;

    let mut chunks: Vec<u64> = vec![CHUNK_SIZE; (content_length / CHUNK_SIZE) as usize];
    chunks.push(content_length % CHUNK_SIZE);
    Ok(chunks)
}

#[tokio::main]
async fn main() {
    let url = "http://localhost:3000/sfsymbols.json";
    // let url = "https://speed.hetzner.de/100MB.bin";

    match get_chunk_vec(url).await {
        Ok(val) => {
            println!("Chunks {:?}", val);
        }

        Err(err) => {
            if let Some(_err) = err.downcast_ref::<reqwest::Error>() {
                println!("Error occurred: {}", _err.is_status());
            }
        }
    }
}
