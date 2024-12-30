//! Heavy metal notifier will notify you via RSS whenever there are new heavy metal album releases.
//!
//! The application works by reading the heavy metal album releases throughout the year from
//! [Wikipedia's heavy metal releases](https://en.wikipedia.org/wiki/2024_in_heavy_metal_music)
//! page and [Metallum](https://www.metal-archives.com/release/upcoming). It is updated at 12:00 AM,
//! on day 1 and 15 of the month.

mod calendar;
mod error;
mod scraper;
mod support;

pub mod config;
pub mod jobs;
pub mod model;
pub mod web;

pub use error::{Error, Result};
use time::OffsetDateTime;

/// Returns the current date and time.
pub fn date_now() -> OffsetDateTime {
    OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc())
}

#[cfg(test)]
mod tests {
    use super::*;

    type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn test_date_now_ok() -> Result<()> {
        let now = OffsetDateTime::now_local()?;

        let got = date_now();

        assert_eq!(got.day(), now.day());
        assert_eq!(got.month(), now.month());
        assert_eq!(got.year(), now.year());
        Ok(())
    }
}
