//! The `model` module implements the building blocks of the repository.

mod calendar;
mod entities;
mod feed;
mod store;

pub(in crate::model) mod schema;

pub use calendar::{Artist, CalendarBmc, CalendarRepository, Release};
pub use entities::{EntitiesBmc, EntitiesRepository};
pub use feed::{Feed, FeedBmc, FeedRepository};

use crate::config::config;
use diesel::prelude::*;
use store::establish_connection;

/// `ModelManager` is a structure responsible for managing database interactions.
pub struct ModelManager {
    /// The connection to the SQLite database.
    pub conn: SqliteConnection,
}

impl ModelManager {
    pub fn new() -> Self {
        Self {
            conn: establish_connection(&config().DATABASE_URL),
        }
    }
}

impl Default for ModelManager {
    fn default() -> Self {
        Self::new()
    }
}
