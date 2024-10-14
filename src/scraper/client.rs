use reqwest::Url;
use scraper::Html;
use tracing::error;

use super::metallum::MetallumReleases;
use crate::error::Result;

pub struct MainClient;

impl MainClient {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait Client {
    fn get_calendar(&self, year: i32) -> Result<scraper::Html>;
    fn get_bandcamp_link(&self, artist: String) -> Option<Url>;
    fn fetch_metallum(&self, page: u16) -> Option<MetallumReleases>;
}

impl Client for MainClient {
    fn get_calendar(&self, year: i32) -> Result<scraper::Html> {
        let url = format!("https://en.wikipedia.org/wiki/{year}_in_heavy_metal_music");
        let res = reqwest::blocking::get(url)?;
        let text = res.text()?;
        Ok(Html::parse_document(text.as_str()))
    }

    fn get_bandcamp_link(&self, artist: String) -> Option<Url> {
        let artist = artist
            .to_lowercase()
            .replace(":", "")
            .split_whitespace()
            .collect::<String>();
        let url = format!("https://{artist}.bandcamp.com");
        let res = match reqwest::blocking::get(&url) {
            Ok(res) => {
                let req_url = res.url().path();
                let req_host = res.url().host_str().unwrap_or("");
                if req_url != "/signup" || req_host == format!("{}.bandcamp.com", artist) {
                    Some(Url::parse(&url).unwrap())
                } else {
                    None
                }
            }
            Err(_) => None,
        };
        std::thread::sleep(std::time::Duration::from_millis(200));
        res
    }

    fn fetch_metallum(&self, page: u16) -> Option<MetallumReleases> {
        let page = page * 100;
        let url = format!("https://www.metal-archives.com/release/ajax-upcoming/json/1?sEcho=3&iColumns=6&sColumns=&iDisplayStart={page}&iDisplayLength=100&mDataProp_0=0&mDataProp_1=1&mDataProp_2=2&mDataProp_3=3&mDataProp_4=4&mDataProp_5=5&iSortCol_0=4&sSortDir_0=asc&iSortingCols=1&bSortable_0=true&bSortable_1=true&bSortable_2=true&bSortable_3=true&bSortable_4=true&bSortable_5=true&includeVersions=0&fromDate=2024-10-03&toDate=0000-00-00");

        match reqwest::blocking::get(&url) {
            Ok(res) => {
                let res: core::result::Result<MetallumReleases, serde_json::Error> =
                    serde_json::from_reader(res);
                match res {
                    Ok(releases) => {
                        if releases.data.is_empty() {
                            None
                        } else {
                            Some(releases)
                        }
                    }
                    Err(err) => {
                        error!("Failed to decode response: {err}; page={page}; url={url}");
                        None
                    }
                }
            }
            Err(err) => {
                error!("Failed to fetch metallum releases: {err}; page={page}; url={url}");
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

        pub fn scrape(&self, year: i32) -> Result<Calendar> {
            scrape(self, year)
        }
    }

    impl Client for MockClient {
        fn get_calendar(&self, year: i32) -> Result<scraper::Html> {
            let path = PathBuf::from(format!("./tests/testdata/wiki/test_{year}.html"));

            let content = match fs::read_to_string(&path) {
                Ok(content) => content,
                Err(_) => {
                    let url = format!("https://en.wikipedia.org/wiki/{year}_in_heavy_metal_music");
                    match reqwest::blocking::get(url) {
                        Ok(res) => {
                            let mut file = fs::File::create(path)?;
                            let content = res.text()?;
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

        fn get_bandcamp_link(&self, artist: String) -> Option<Url> {
            let artist = artist
                .to_lowercase()
                .replace(":", "")
                .split_whitespace()
                .collect::<String>();
            let url = format!("https://{artist}.bandcamp.com");
            println!("{url}");
            Some(Url::parse(&url).unwrap())
        }

        fn fetch_metallum(&self, page: u16) -> Option<MetallumReleases> {
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
