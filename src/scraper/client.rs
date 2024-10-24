use axum::async_trait;
use reqwest::Url;
use scraper::Html;
use time::OffsetDateTime;
use tracing::error;

use super::metallum::MetallumReleases;
use crate::error::Result;

pub struct MainClient {
    http_client: reqwest::Client,
}

impl MainClient {
    pub fn new(http_client: reqwest::Client) -> Self {
        Self { http_client }
    }
}

#[async_trait]
pub trait Client {
    async fn get_calendar(&self, year: i32) -> Result<scraper::Html>;
    async fn get_bandcamp_link(&self, artist: String) -> Option<Url>;
    async fn fetch_metallum(&self, page: u16) -> Option<MetallumReleases>;
}

#[async_trait]
impl Client for MainClient {
    async fn get_calendar(&self, year: i32) -> Result<scraper::Html> {
        let url = format!("https://en.wikipedia.org/wiki/{year}_in_heavy_metal_music");
        let res = self.http_client.get(url).send().await?;
        let text = res.text().await?;
        Ok(Html::parse_document(text.as_str()))
    }

    async fn get_bandcamp_link(&self, artist: String) -> Option<Url> {
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        let artist = artist
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect::<String>();

        let url = format!("https://{artist}.bandcamp.com");

        match self.http_client.get(&url).send().await {
            Ok(res) => {
                let is_valid = res
                    .url()
                    .host_str()
                    .map(|host| host == format!("{}.bandcamp.com", artist))
                    .unwrap_or(false)
                    && res.url().path() != "/signup";

                is_valid.then(|| Url::parse(&url).ok()).flatten()
            }
            Err(err) => {
                error!("artist = {artist}; url = {url}; err = {err}");
                None
            }
        }
    }

    async fn fetch_metallum(&self, page: u16) -> Option<MetallumReleases> {
        let offset = page * 100;
        let now = OffsetDateTime::now_utc();
        let from_date = format!("{}-{}-{}", now.year(), now.month() as u8, now.day());
        let url = format!("https://www.metal-archives.com/release/ajax-upcoming/json/1?sEcho=3&iColumns=6&sColumns=&iDisplayStart={offset}&iDisplayLength=100&mDataProp_0=0&mDataProp_1=1&mDataProp_2=2&mDataProp_3=3&mDataProp_4=4&mDataProp_5=5&iSortCol_0=4&sSortDir_0=asc&iSortingCols=1&bSortable_0=true&bSortable_1=true&bSortable_2=true&bSortable_3=true&bSortable_4=true&bSortable_5=true&includeVersions=0&fromDate={from_date}&toDate=0000-00-00");

        match self.http_client.get(&url).send().await {
            Ok(res) => {
                let body = res.bytes().await.ok()?;
                let res: core::result::Result<MetallumReleases, serde_json::Error> =
                    serde_json::from_slice(&body);
                match res {
                    Ok(releases) => {
                        if releases.data.is_empty() {
                            None
                        } else {
                            Some(releases)
                        }
                    }
                    Err(err) => {
                        error!("Failed to decode response: {err}; offset={offset}; url={url}");
                        None
                    }
                }
            }
            Err(err) => {
                error!("Failed to fetch metallum releases: {err}; offset={offset}; url={url}");
                None
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use std::{fs, io::Write, path::PathBuf};

    use crate::{calendar::Calendar, error::Error, scraper::wiki::scrape};

    #[cfg(test)]
    pub struct MockClient;

    #[cfg(test)]
    impl MockClient {
        pub fn new() -> Self {
            Self {}
        }

        pub async fn scrape(&self, year: i32) -> Result<Calendar> {
            scrape(self, year).await
        }
    }

    #[async_trait]
    impl Client for MockClient {
        async fn get_calendar(&self, year: i32) -> Result<scraper::Html> {
            let path = PathBuf::from(format!("./tests/testdata/wiki/test_{year}.html"));

            let content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(_) => {
                    let url = format!("https://en.wikipedia.org/wiki/{year}_in_heavy_metal_music");
                    match reqwest::get(url).await {
                        Ok(res) => {
                            let mut file = fs::File::create(path)?;
                            let content = res.text().await?;
                            if let Err(err) = file.write(&content.as_bytes()) {
                                return Err(Error::Io(err));
                            }
                            content
                        }
                        Err(_) => return Err(Error::RequestFail),
                    }
                }
            };

            Ok(Html::parse_document(&content))
        }

        async fn get_bandcamp_link(&self, artist: String) -> Option<Url> {
            let artist = artist
                .to_lowercase()
                .replace(":", "")
                .split_whitespace()
                .collect::<String>();
            let url = format!("https://{artist}.bandcamp.com");
            println!("{url}");
            Some(Url::parse(&url).unwrap())
        }

        async fn fetch_metallum(&self, page: u16) -> Option<MetallumReleases> {
            let page = page * 100 + 100;
            let path_str = format!("./tests/testdata/metallum/{page}.json");
            let path = PathBuf::from(&path_str);

            match fs::read_to_string(&path) {
                Ok(content) => {
                    let res: core::result::Result<MetallumReleases, serde_json::Error> =
                        serde_json::from_str(&content);
                    match res {
                        Ok(releases) => {
                            if releases.data.is_empty() {
                                None
                            } else {
                                Some(releases)
                            }
                        }
                        Err(err) => {
                            error!(
                                "Failed to decode response: {err}; page={page}; path={}",
                                &path_str
                            );
                            None
                        }
                    }
                }
                Err(err) => {
                    error!(
                        "Failed to fetch metallum releases: {err}; page={page}; path={}",
                        &path_str
                    );
                    None
                }
            }
        }
    }
}
