use diesel::prelude::*;

use super::{schema, ModelManager};
use crate::error::Result;

/// Represents a row in the `feeds` table, providing access to
/// the RSS feed data stored in the SQLite database.
#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = super::schema::feeds)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Feed {
    pub id: i32,
    /// The date when the feed was published.
    pub date: i32,
    /// The content of the RSS feed.
    pub feed: String,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::feeds)]
#[diesel(belongs_to(super::schema::custom_feeds))]
struct FeedForInsert {
    pub date: i32,
    pub feed: String,
    pub custom_feed_id: i32,
}

/// Represents a row in the `custom_feeds` table, providing access to
/// the custom RSS feed data stored in the SQLite database.
#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = super::schema::custom_feeds)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CustomFeed {
    pub id: i32,
    pub bands: String,
    pub genres: String,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::custom_feeds)]
struct CustomFeedForInsert {
    pub bands: String,
    pub genres: String,
}

/// A trait defining the interface for querying a entities of heavy metal releases.
///
/// It can be implemented by any backend service or repository pattern to support
// different data storage and retrieval strategies.
pub trait FeedRepository {
    /// Creates a new feed record in the database using the provided `FeedForCreate` data.
    ///
    /// This method accepts a `FeedForCreate` object and inserts it into the `feeds` table.
    /// The insert operation is ignored if a record with the same data already exists.
    fn create(&self, date_c: i32, feed_c: &str, custom_feed: i32) -> Result<()>;

    /// Retrieves the most recent feed records from the database.
    ///
    /// This method fetches a limited number of feed records from the
    /// `feeds` table, ordered by date in descending order.
    fn get(&self, num: i64, custom_feed: i32) -> Result<Vec<Feed>>;

    /// Retrieves a `CustomFeed` by its ID.
    fn get_custom_feed(&self, custom_feed_id: i32) -> Result<CustomFeed>;

    /// Retrieves or creates a custom feed based on the specified bands and genres.
    ///
    /// This function first normalizes the input bands and genres vectors:
    /// - If `bands_vec` contains "All", it is cleared (no specific bands filter).
    /// - If `bands_vec` contains "None", it is replaced with a single "none" entry.
    /// - If `genres_vec` contains "All", it is cleared (no specific genres filter).
    /// - If `genres_vec` contains "None", it is replaced with a single "none" entry.
    ///
    fn get_or_create_custom_feed(
        &self,
        bands_vec: Vec<String>,
        genres_vec: Vec<String>,
    ) -> Option<i32>;
}

/// `FeedBmc` is a backend model controller responsible for handling
/// feed-related operations in the application.
///
/// It provides methods to create and retrieve feed records from the database.
pub struct FeedBmc;

impl FeedRepository for FeedBmc {
    fn create(&self, date_c: i32, feed_c: &str, custom_feed: i32) -> Result<()> {
        use schema::feeds::dsl::*;

        diesel::insert_or_ignore_into(feeds)
            .values(&FeedForInsert {
                date: date_c,
                feed: feed_c.into(),
                custom_feed_id: custom_feed,
            })
            .execute(&mut ModelManager::new().conn)?;

        Ok(())
    }

    fn get(&self, num: i64, custom_feed: i32) -> Result<Vec<Feed>> {
        use schema::feeds::dsl::*;

        let results = feeds
            .filter(custom_feed_id.eq(custom_feed))
            .order(date.desc())
            .limit(num)
            .select(Feed::as_select())
            .load(&mut ModelManager::new().conn)?;

        Ok(results)
    }

    fn get_custom_feed(&self, custom_feed_id: i32) -> Result<CustomFeed> {
        use schema::custom_feeds::dsl::*;

        let feed = custom_feeds
            .filter(id.eq(custom_feed_id))
            .first::<CustomFeed>(&mut ModelManager::new().conn)?;

        Ok(feed)
    }

    fn get_or_create_custom_feed(
        &self,
        mut bands_vec: Vec<String>,
        mut genres_vec: Vec<String>,
    ) -> Option<i32> {
        use schema::custom_feeds::dsl::*;

        let all = "ALL".to_string();
        let none = "None".to_string();

        bands_vec = match bands_vec.as_slice() {
            vec if vec.contains(&all) => Vec::new(),
            vec if vec.contains(&none) => vec!["none".to_string()],
            _ => bands_vec,
        };

        genres_vec = match genres_vec.as_slice() {
            vec if vec.contains(&all) => Vec::new(),
            vec if vec.contains(&none) => vec!["none".to_string()],
            _ => genres_vec
                .into_iter()
                .map(|s| s.to_lowercase().replace(" metal", ""))
                .collect(),
        };

        if bands_vec.is_empty() && genres_vec.is_empty() {
            return None;
        }

        let bands_all = bands_vec.join("@").to_lowercase();
        let genres_all = genres_vec.join("@").to_lowercase();

        let conn = &mut ModelManager::new().conn;

        custom_feeds
            .filter(bands.eq(&bands_all).and(genres.eq(&genres_all)))
            .select(id)
            .first::<i32>(conn)
            .optional()
            .ok()?
            .or_else(|| {
                diesel::insert_into(custom_feeds)
                    .values(&CustomFeedForInsert {
                        bands: bands_all,
                        genres: genres_all,
                    })
                    .returning(id)
                    .get_result::<i32>(conn)
                    .ok()
            })
    }
}
