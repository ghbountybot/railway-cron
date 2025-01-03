use reqwest::blocking::Client;
use std::env;
use tracing::{debug, instrument};

#[derive(Debug)]
pub struct UrlResponse {
    pub url: String,
    pub status: u16,
    pub body: String,
}

/// Fetches responses from URLs sequentially using blocking requests
#[instrument]
pub fn fetch_urls() -> anyhow::Result<()> {
    let urls_str = env::var("URLS").expect("URLS must be set in .env file");

    let client = Client::new();

    for url in urls_str.split(',') {
        let url = url.trim();
        if url.is_empty() {
            continue;
        }
        debug!("Fetching {}", url);

        let response = client.get(url).send()?;
        let body = response.error_for_status()?.text()?;
        debug!("Response body: {body}");
    }

    Ok(())
}
