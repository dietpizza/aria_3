use {
    anyhow::{Context, Result},
    md5,
    reqwest::Client,
    std::path::Path,
};

const CHUNK_SIZE: u64 = 4 * 1024 * 1024;

#[derive(Debug)]
pub struct Download {
    id: String,
    length: u64,
    url: String,
    parent_dir: String,
    filepath: String,
    is_resumable: bool,
    ranges: Vec<(u64, u64)>,
}

impl Download {
    pub fn new(url: &str, filepath: &str) -> Download {
        Download {
            // User provided
            url: url.to_string(),
            filepath: filepath.to_string(),

            // Defaults
            id: "".to_owned(),
            length: 0,
            ranges: vec![],
            is_resumable: false,
            parent_dir: "".to_owned(),
        }
    }
    pub async fn connect(&mut self) -> Result<()> {
        (self.length, self.is_resumable) = get_length(&self.url).await?;
        self.ranges = gen_ranges(self.length).await?;
        self.id = format!("{:x}", md5::compute(&self.url));

        self.parent_dir = Path::new(&self.filepath)
            .parent()
            .context("Failed to parse filepath")?
            .to_str()
            .context("Failed to parse Path")?
            .to_owned();

        Ok(())
    }
}

async fn gen_ranges(length: u64) -> Result<Vec<(u64, u64)>> {
    let mut chunk_map: Vec<(u64, u64)> = vec![];
    let mut chunk_vec = vec![CHUNK_SIZE; (length / CHUNK_SIZE) as usize];
    let rem = length % CHUNK_SIZE;
    if rem > 0 {
        chunk_vec.push(rem);
    }
    for i in 0..chunk_vec.len() {
        let end = chunk_vec
            .get(i)
            .context(format!("Failed to get {i}"))?
            .clone()
            * (i as u64);
        let start: u64 = if i > 0 {
            chunk_vec
                .get(i - 1)
                .context(format!("Failed to get {i}"))?
                .clone()
                * ((i - 1) as u64)
        } else {
            0
        };

        chunk_map.push((start, end));
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
