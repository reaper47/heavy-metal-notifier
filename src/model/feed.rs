use diesel::prelude::*;

use crate::error::Result;

use super::{schema, ModelManager};

/// `Feed` represents a row in the `feeds` table, providing access to
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

/// `FeedForCreate` is a simplified version of the [`Feed`](struct.Feed.html) struct
/// used when creating new feed entries.
///
/// Unlike `Feed`, this struct does not include the `id` field, as the database
/// will generate it automatically when the record is inserted.
pub struct FeedForCreate {
    /// The date when the feed was published.
    pub date: i32,
    /// The content of the RSS feed.
    pub feed: String,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::feeds)]
struct FeedForInsert {
    pub date: i32,
    pub feed: String,
}

/// `FeedBmc` is a backend model controller responsible for handling
/// feed-related operations in the application.
///
/// It provides methods to create and retrieve feed records from the database.
pub struct FeedBmc;

impl FeedBmc {
    /// Creates a new feed record in the database using the provided `FeedForCreate` data.
    ///
    /// This method accepts a `FeedForCreate` object and inserts it into the `feeds` table.
    /// The insert operation is ignored if a record with the same data already exists.
    pub fn create(feed_c: FeedForCreate) -> Result<()> {
        use schema::feeds::dsl::*;

        let mm = &mut ModelManager::new();
        let conn = &mut mm.conn;

        diesel::insert_or_ignore_into(feeds)
            .values(&FeedForInsert {
                date: feed_c.date,
                feed: feed_c.feed,
            })
            .execute(conn)?;

        Ok(())
    }

    /// Retrieves the most recent feed records from the database.
    ///
    /// This method fetches a limited number of feed records from the
    /// `feeds` table, ordered by date in descending order.
    pub fn get(num: i64) -> Result<Vec<Feed>> {
        use schema::feeds::dsl::*;

        let mm = &mut ModelManager::new();
        let conn = &mut mm.conn;

        let results = feeds
            .order(date.desc())
            .limit(num)
            .select(Feed::as_select())
            .load(conn)?;

        Ok(results)
    }
}
