use diesel::prelude::*;

use super::{schema, ModelManager};
use crate::error::Result;

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

#[derive(Insertable)]
#[diesel(table_name = super::schema::feeds)]
#[diesel(belongs_to(super::schema::custom_feeds))]
struct FeedForInsert {
    pub date: i32,
    pub feed: String,
    pub custom_feed_id: i32,
}

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
    pub fn create(date_c: i32, feed_c: impl Into<String>, custom_feed: i32) -> Result<()> {
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

    /// Retrieves the most recent feed records from the database.
    ///
    /// This method fetches a limited number of feed records from the
    /// `feeds` table, ordered by date in descending order.
    pub fn get(num: i64, custom_feed: i32) -> Result<Vec<Feed>> {
        use schema::feeds::dsl::*;

        let results = feeds
            .filter(custom_feed_id.eq(custom_feed))
            .order(date.desc())
            .limit(num)
            .select(Feed::as_select())
            .load(&mut ModelManager::new().conn)?;

        Ok(results)
    }

    pub fn get_custom_feed(custom_feed_id: i32) -> Result<CustomFeed> {
        use schema::custom_feeds::dsl::*;

        let feed = custom_feeds
            .filter(id.eq(custom_feed_id))
            .first::<CustomFeed>(&mut ModelManager::new().conn)?;

        Ok(feed)
    }

    pub fn get_or_create_custom_feed(
        bands_vec: Vec<String>,
        genres_vec: Vec<String>,
    ) -> Option<i32> {
        use schema::custom_feeds::dsl::*;

        let all = String::from("All");
        let none = String::from("None");

        let bands_vec = if bands_vec.contains(&all) {
            Vec::new()
        } else if bands_vec.contains(&none) {
            vec!["none".to_string()]
        } else {
            bands_vec
        };

        let genres_vec = if genres_vec.contains(&all) {
            Vec::new()
        } else if   genres_vec.contains(&none) {
            vec!["none".to_string()]
        } else {
            genres_vec
                .iter()
                .map(|s| s.to_lowercase().replace(" metal", ""))
                .collect()
        };

        if bands_vec.is_empty() && genres_vec.is_empty() {
            return None;
        }

        let bands_all = bands_vec.join("@").to_lowercase();
        let genres_all = genres_vec.join("@").to_lowercase();

        let conn = &mut ModelManager::new().conn;
        custom_feeds
            .filter(bands.eq(&bands_all).and(genres.eq(&genres_all)))
            .limit(1)
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
                    .optional()
                    .ok()?
            })
    }
}
