use diesel::prelude::*;
use tracing::{error, info, warn};

use super::ModelManager;
use crate::{
    calendar::Calendar,
    config::config,
    date_now,
    error::{Error, Result},
    scraper::client::Client,
};

/// This struct corresponds to a row in the `artists` table in the database.
/// Each artist has a unique `id` and a `name`.
#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, AsChangeset)]
#[diesel(table_name = super::schema::artists)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Artist {
    pub id: i32,
    pub name: String,
    pub genre: Option<String>,
    pub url_bandcamp: Option<String>,
    pub url_metallum: Option<String>,
}

/// Represents a new artist to be inserted into the database.
///
/// This struct is used when creating new records in the `artists` table.
/// It doesn't include the `id` field because the database will generate it.
#[derive(Insertable)]
#[diesel(table_name = super::schema::artists)]
struct ArtistForInsert {
    pub name: String,
    pub genre: Option<String>,
    pub url_bandcamp: Option<String>,
    pub url_metallum: Option<String>,
}

impl ArtistForInsert {
    pub fn new(
        name: impl Into<String>,
        genre: Option<String>,
        url_metallum: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            genre,
            url_bandcamp: None,
            url_metallum,
        }
    }
}

/// Represents a music release by an artist.
///
/// This struct corresponds to a row in the `releases` table.
/// It stores information about an artist's album release,
/// including the release date (year, month, day) and the album's
/// title.
#[derive(Queryable, Identifiable, Selectable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Artist))]
#[diesel(table_name = super::schema::releases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Release {
    pub id: i32,
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub artist_id: i32,
    pub album: String,
    pub release_type: Option<String>,
    pub url_youtube: String,
    pub url_metallum: Option<String>,
}

impl Release {
    /// Converts the release and associated artist information into an HTML string.
    ///
    /// This function generates a `<li>` element containing the release's title and the artist's name,
    /// followed by a nested `<ul>` list. The list includes optional details such as:
    /// - The artist's genre, if available.
    /// - The type of release (e.g., album, single), if specified.
    /// - Links to YouTube, Bandcamp, and Metallum pages related to the artist or release.
    pub fn to_html(&self, artist: &Artist) -> String {
        let mut html = format!(
            "<li style=\"margin-bottom: 1rem\"><b>{} - {}</b>",
            artist.name, self.album
        );

        html.push_str("<ul>");
        if let Some(genre) = &artist.genre {
            html.push_str(&format!("<li>{genre}</li>"));
        }

        if let Some(release_type) = &self.release_type {
            html.push_str(&format!("<li>{release_type}</li>"));
        }

        html.push_str(&format!(
            "<li><a href=\"{}\" target=\"_blank\" class=\"link link-primary visited:link-secondary focus:link-accent\">Youtube</a></li>",
            self.url_youtube
        ));

        if let Some(url) = &artist.url_bandcamp {
            html.push_str(&format!(
                "<li><a href=\"{url}\" target=\"_blank\" class=\"link link-primary visited:link-secondary focus:link-accent\">Bandcamp</a></li>"
            ));
        }

        if let Some(url) = &artist.url_metallum {
            html.push_str(&format!(
                "<li><a href=\"{url}\" target=\"_blank\" class=\"link link-primary visited:link-secondary focus:link-accent\">Metallum (band)</a></li>"
            ));
        }

        if let Some(url) = &self.url_metallum {
            html.push_str(&format!(
                "<li><a href=\"{url}\" target=\"_blank\" class=\"link link-primary visited:link-secondary focus:link-accent\">Metallum (album)</a></li>"
            ));
        }

        html.push_str("</ul></li>");
        html
    }
}

/// Represents a new release to be inserted into the database.
///
/// This struct is used when creating new records in the `releases` table.
/// It doesn't include the `id` field because the database will generate it.
#[derive(Insertable, Associations)]
#[diesel(belongs_to(Artist))]
#[diesel(table_name = super::schema::releases)]
struct ReleaseForInsert {
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub artist_id: i32,
    pub album: String,
    pub release_type: Option<String>,
    pub url_youtube: String,
    pub url_metallum: Option<String>,
}

#[axum::async_trait]
/// A trait defining the interface for managing and querying a calendar of heavy metal releases.
///
/// It can be implemented by any backend service or repository pattern to support
// different data storage and retrieval strategies.
pub trait CalendarRepository {
    /// Creates or updates a calendar with the provided data.
    ///
    /// This method inserts new releases into the `releases` table
    /// or updates existing ones based on the calendar data. It
    /// handles linking artists and adding external links (YouTube, Bandcamp).
    async fn create_or_update(&self, calendar: Calendar) -> Result<()>;

    /// Retrieves releases for the current date.
    ///
    /// This method fetches releases from the `releases` table
    /// that match the current date (year, month, and day) and
    /// joins the associated artist and links (YouTube, Bandcamp).
    fn get(&self) -> Result<Vec<(Release, Artist)>>;

    /// Retrieves the releases for the given date from the database.
    ///
    /// This method fetches a limited number of feed records from the
    /// `feeds` table, ordered by date in descending order.
    fn get_by_date(
        &self,
        target_year: u32,
        target_month: u8,
        target_day: u8,
    ) -> Result<Vec<(Release, Artist)>>;

    /// Fetches the number of releases for the given date.
    fn fetch_releases(
        &self,
        target_year: u32,
        target_month: u8,
        target_day: u8,
    ) -> Result<Vec<(Release, Artist)>>;

    /// Returns the number of releases for a specific date, if any.
    fn num_releases(&self, target_year: u32, target_month: u8, target_day: u8) -> Option<i64>;

    /// Asynchronously updates Bandcamp URLs for artists missing them in the database.
    ///
    /// This function fetches Bandcamp links for artists whose `url_bandcamp` field is `NULL`
    /// and updates the corresponding records in the database. The function only runs in
    /// production mode. If not, it logs a warning and exits early.
    ///
    /// # Errors
    ///
    /// This function returns an error if:
    /// - There is an issue connecting to or querying the database.
    /// - Updating the artist records in the database fails.
    /// - Fetching Bandcamp links encounters an error.
    ///
    async fn update_bandcamp(&self, client: &(dyn Client + Sync)) -> Result<()>;
}

/// `CalendarBmc` is a backend model controller responsible for
/// managing calendar-related operations.
///
/// It provides methods to create, update, and retrieve calendar
/// data, including releases and associated links.
pub struct CalendarBmc;

#[axum::async_trait]
impl CalendarRepository for CalendarBmc {
    async fn create_or_update(&self, calendar: Calendar) -> Result<()> {
        use super::schema::*;

        let conn = &mut ModelManager::new().conn;
        conn.transaction::<_, Error, _>(|conn| {
            diesel::delete(releases::table.filter(releases::year.eq(calendar.year)))
                .execute(conn)?;

            for (month, data) in calendar.data.iter() {
                for (day, releases) in data.iter() {
                    for release in releases.iter() {
                        let artist_name = release.artist.clone();

                        let genre = release
                            .metallum_info
                            .as_ref()
                            .map(|info| info.genre.clone());

                        let url_metallum = release
                            .metallum_info
                            .as_ref()
                            .map(|info| info.artist_link.clone());

                        let artist_id: i32 = match diesel::insert_or_ignore_into(artists::table)
                            .values(&ArtistForInsert::new(&artist_name, genre, url_metallum))
                            .returning(artists::id)
                            .get_result(conn)
                        {
                            Ok(id) => id,
                            Err(_) => artists::table
                                .filter(artists::name.eq(&artist_name))
                                .limit(1)
                                .select(artists::id)
                                .get_result(conn)?,
                        };

                        let query = format!("{} {} full album", artist_name, release.album.clone());
                        let mut query_encoded = String::new();
                        url_escape::encode_query_to_string(query, &mut query_encoded);

                        diesel::insert_into(releases::table)
                            .values(&ReleaseForInsert {
                                year: calendar.year,
                                month: *month as i32,
                                day: *day as i32,
                                artist_id,
                                album: release.album.clone(),
                                release_type: release
                                    .metallum_info
                                    .as_ref()
                                    .map(|info| info.release_type.clone()),
                                url_youtube: format!(
                                    "https://www.youtube.com/results?search_query={query_encoded}"
                                ),
                                url_metallum: release
                                    .metallum_info
                                    .as_ref()
                                    .map(|info| info.album_link.clone()),
                            })
                            .execute(conn)?;
                    }
                }
            }

            Ok(())
        })
    }

    fn get(&self) -> Result<Vec<(Release, Artist)>> {
        let now = date_now();

        let releases = self.fetch_releases(now.year() as u32, now.month() as u8, now.day())?;

        Ok(releases)
    }

    fn get_by_date(
        &self,
        target_year: u32,
        target_month: u8,
        target_day: u8,
    ) -> Result<Vec<(Release, Artist)>> {
        let releases = self.fetch_releases(target_year, target_month, target_day)?;

        Ok(releases)
    }

    fn fetch_releases(
        &self,
        target_year: u32,
        target_month: u8,
        target_day: u8,
    ) -> Result<Vec<(Release, Artist)>> {
        use super::schema::{artists::dsl::*, releases::dsl::*};

        let results = releases
            .inner_join(artists)
            .filter(
                year.eq(target_year as i32)
                    .and(month.eq(target_month as i32))
                    .and(day.eq(target_day as i32)),
            )
            .order(name.asc())
            .select((Release::as_select(), Artist::as_select()))
            .load(&mut ModelManager::new().conn)?;

        Ok(results)
    }

    fn num_releases(&self, target_year: u32, target_month: u8, target_day: u8) -> Option<i64> {
        use super::schema::releases::dsl::*;

        releases
            .filter(
                year.eq(target_year as i32)
                    .and(month.eq(target_month as i32))
                    .and(day.eq(target_day as i32)),
            )
            .count()
            .get_result(&mut ModelManager::new().conn)
            .map_err(|err| error!("Failed to fetch num_releases in StatisticsBmc: {err}"))
            .ok()
            .filter(|&num| num > 0)
    }

    async fn update_bandcamp(&self, client: &(dyn Client + Sync)) -> Result<()> {
        use super::schema::*;

        if !config().IS_PROD {
            warn!("Can only fetch Bandcamp links when in production.");
            return Ok(());
        }

        let conn = &mut ModelManager::new().conn;

        let mut all_artists: Vec<Artist> = artists::table
            .filter(artists::url_bandcamp.is_null())
            .select(Artist::as_select())
            .load(conn)?;

        info!("Fetching {} Bandcamp links", all_artists.len());

        let mut num_success = 0;
        for artist in &mut all_artists {
            artist.url_bandcamp = client
                .get_bandcamp_link(artist.name.clone())
                .await
                .map(|url| url.to_string());

            if artist.url_bandcamp.is_some() {
                num_success += 1;
            }
        }

        info!(
            "{num_success}/{} artists have a Bandcamp page.",
            all_artists.len()
        );

        for artist in &all_artists {
            diesel::update(artists::table.find(artist.id))
                .set(artist)
                .execute(conn)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_release_all_fields_to_html_ok() {
        let artist = Artist {
            id: 1,
            name: String::from("Wintersun"),
            genre: Some(String::from("Symphonic Melodic Death Metal")),
            url_bandcamp: Some(String::from("https://wintersun.bandcamp.com")),
            url_metallum: Some(String::from(
                "https://www.metal-archives.com/band/wintersun",
            )),
        };
        let release = Release {
            id: 1,
            year: 2024,
            month: 8,
            day: 31,
            artist_id: 1,
            album: String::from("Time II"),
            release_type: Some(String::from("Full-Length")),
            url_youtube: String::from("https://www.youtube.com"),
            url_metallum: Some(String::from("https://www.metal-archives.com")),
        };

        let got = release.to_html(&artist);

        let want = "<li style=\"margin-bottom: 1rem\"><b>Wintersun - Time II</b><ul><li>Symphonic Melodic Death Metal</li><li>Full-Length</li><li><a href=\"https://www.youtube.com\" target=\"_blank\" class=\"link link-primary visited:link-secondary focus:link-accent\">Youtube</a></li><li><a href=\"https://wintersun.bandcamp.com\" target=\"_blank\" class=\"link link-primary visited:link-secondary focus:link-accent\">Bandcamp</a></li><li><a href=\"https://www.metal-archives.com/band/wintersun\" target=\"_blank\" class=\"link link-primary visited:link-secondary focus:link-accent\">Metallum (band)</a></li><li><a href=\"https://www.metal-archives.com\" target=\"_blank\" class=\"link link-primary visited:link-secondary focus:link-accent\">Metallum (album)</a></li></ul></li>";
        pretty_assertions::assert_eq!(got, want);
    }
}
