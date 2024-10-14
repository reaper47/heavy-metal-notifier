//! Heavy metal notifier Our project will notify you via RSS whenever there are
//! new heavy metal album releases.
//!
//! The application works by creating a calendar from [Wikipedia heavy metal releases](https://en.wikipedia.org/wiki/2024_in_heavy_metal_music)
//! page that lists all the heavy metal album releases throughout the year. It
//! is updated at 12:00 AM, on day 1 and 15 of the month.

mod calendar;
mod config;
mod error;
mod scraper;
mod support;

pub mod jobs;
pub mod model;
pub mod web;

pub use error::{Error, Result};
