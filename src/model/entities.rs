use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::model::ModelManager;

/// A trait defining the interface for querying a entities of heavy metal releases.
///
/// It can be implemented by any backend service or repository pattern to support
// different data storage and retrieval strategies.
pub trait EntitiesRepository {
    /// Fetches and returns a sorted list of band names from the database.
    fn bands(&self) -> Vec<String>;
}

/// `EntitiesBmc` is a backend model controller responsible for
/// querying what belongs to heavy metal music.
pub struct EntitiesBmc;

impl EntitiesRepository for EntitiesBmc {
    fn bands(&self) -> Vec<String> {
        use super::schema::artists::dsl::*;

        artists
            .select(name)
            .order(name.asc())
            .load::<String>(&mut ModelManager::new().conn)
            .unwrap_or_else(|_| vec![])
    }
}
