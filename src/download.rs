use std::{fs::File, io::Write};

use reqwest::header;

use {
    anyhow::{Context, Result},
    md5,
    rand::Rng,
    reqwest::Client,
    std::path::Path,
    std::sync::{Arc, Mutex},
    tokio::time::{sleep, Duration},
};

const CHUNK_SIZE: u64 = 4 * 1024 * 1024;

#[derive(Debug)]
pub struct Chunk {
    start: u64,
    end: u64,
    offset: u64,
    is_downloaded: bool,
    path: String,
    url: String,
}

impl Chunk {
    pub fn new(start: u64, end: u64, url: &str, path: &str) -> Chunk {
        Chunk {
            start,
            end,
            offset: 0,
            is_downloaded: false,
            url: url.to_owned(),
            path: path.to_owned(),
        }
    }

    pub fn update_offset(&mut self, offset: u64) {
        self.offset = offset;
        if offset + self.start == self.end {
            self.is_downloaded = true;
        }
    }

    pub async fn download_chunk(&mut self) -> Result<bool> {
        let client = Client::new();
        let range_header =
            header::HeaderValue::from_str(&format!("bytes={}-{}", self.start, self.end))?;

        let mut response = client
            .get(&self.url)
            .header(header::RANGE, range_header)
            .send()
            .await?;
        let mut file = File::create(&self.path)?;

        while let Some(chunk) = response.chunk().await? {
            sleep(Duration::from_millis(100)).await;
            file.write_all(&chunk)?;
        }

        Ok(true)
    }
}

#[derive(Debug)]
pub struct Download {
    id: String,
    length: u64,
    url: String,
    parent_dir: String,
    file_path: String,
    is_resumable: bool,
    chunks: Vec<Chunk>,
}

impl Download {
    pub fn new(url: &str, file_path: &str) -> Download {
        Download {
            // User provided
            url: url.to_owned(),
            file_path: file_path.to_owned(),

            // Defaults
            id: "".to_owned(),
            length: 0,
            is_resumable: false,
            parent_dir: "".to_owned(),
            chunks: vec![],
        }
    }
    pub async fn connect(&mut self) -> Result<()> {
        (self.length, self.is_resumable) = get_length(&self.url).await?;
        self.chunks = gen_ranges(self.length, &self.url, &self.file_path).await?;
        self.id = format!("{:x}", md5::compute(&self.url));
        self.parent_dir = Path::new(&self.file_path)
            .parent()
            .context("Failed to parse filepath")?
            .to_str()
            .context("Failed to parse Path")?
            .to_owned();

        Ok(())
    }
}

async fn gen_ranges(length: u64, url: &str, path: &str) -> Result<Vec<Chunk>> {
    let mut chunk_map: Vec<Chunk> = vec![];
    let mut tmp: u64 = 0;

    let _mod = length / CHUNK_SIZE;
    let rem = length % CHUNK_SIZE;

    for _ in 0.._mod {
        chunk_map.push(Chunk::new(tmp, tmp + CHUNK_SIZE - 1, url, path));
        tmp += CHUNK_SIZE;
    }
    if rem > 0 {
        chunk_map.push(Chunk::new(tmp, tmp + rem, url, path));
    }
    Ok(chunk_map)
}

async fn get_length(url: &str) -> Result<(u64, bool)> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let headers = response.headers();

    let accept_ranges = headers.contains_key("Accept-Ranges");
    let content_length: u64 = headers
        .get("Content-Length")
        .context("Content-Length header not found.")?
        .to_str()?
        .parse()
        .context("Unable to parse Content-Length")?;

    // println!("Content-Length {}", file_size);

    Ok((content_length, accept_ranges))
}

pub async fn _run_lol(state: Arc<Mutex<Vec<u64>>>, i: u64) {
    let random_duration = rand::thread_rng().gen_range(100..10000);
    sleep(Duration::from_millis(random_duration)).await;
    let mut _state = state.lock().unwrap();
    _state.retain(|x| x != &i);
}

pub async fn _download_scheduler() {
    let tasks: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(vec![]));
    let mut i: u64 = 0;
    loop {
        sleep(Duration::from_millis(100)).await;
        let tasks_clone = tasks.clone();
        let mut _tasks = tasks.lock().unwrap();
        if _tasks.len() < 4 && i < 100 {
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
