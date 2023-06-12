use std::{
    fs::File,
    io::Write,
    sync::{Arc, Mutex},
};

use {
    anyhow::{Context, Result},
    rand::Rng,
    reqwest::{header, Client},
    tokio::time::{sleep, Duration},
};

const _CHUNK_SIZE: u64 = 4 * 1024 * 1024;

async fn _download_range(url: &str, start: u64, end: u64, file_path: &str) -> Result<bool> {
    let client = Client::new();
    let range_header = header::HeaderValue::from_str(&format!("{start}-{end}"))?;

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

async fn _run_lol(state: Arc<Mutex<Vec<u64>>>, i: u64) {
    let random_duration = rand::thread_rng().gen_range(1..100);
    sleep(Duration::from_millis(random_duration)).await;
    let mut _state = state.lock().unwrap();
    _state.retain(|x| x != &i);
}

async fn _download_scheduler() {
    let tasks: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(vec![]));
    let mut i: u64 = 0;
    loop {
        // sleep(Duration::from_millis(100)).await;
        let tasks_clone = tasks.clone();
        let mut _tasks = tasks.lock().unwrap();
        if _tasks.len() < 100 && i < 100 {
            _tasks.push(i);
            tokio::task::spawn(_run_lol(tasks_clone, i));
            i += 1;
        };
        println!("{:?}", _tasks);
        if _tasks.len() == 0 {
            println!("Task Completed");
            break;
        }
    }
}

async fn _get_cl_header(url: &str) -> Result<u64> {
    let client = Client::new();

    let response = client.get(url).send().await?;

    let content_length: u64 = response
        .headers()
        .get("Content-Length")
        .context("Content-Length header not found.")?
        .to_str()?
        .parse()
        .context("Unable to parse Content-Length")?;

    // println!("Content-Length {}", file_size);

    Ok(content_length)
}

async fn _get_chunk_vec(url: &str) -> Vec<u64> {
    let head_response = _get_cl_header(url).await;

    match head_response {
        Ok(content_length) => {
            let mut chunk_vec: Vec<u64> =
                vec![_CHUNK_SIZE; (content_length / _CHUNK_SIZE) as usize];
            let remainder = content_length % _CHUNK_SIZE;

            if remainder > 0 {
                chunk_vec.push(remainder)
            }
            chunk_vec
        }
        Err(_) => {
            vec![]
        }
    }
}
