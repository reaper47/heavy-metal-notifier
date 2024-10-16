//! The `jobs` module implements functions that are meant to be run periodically.

use time::OffsetDateTime;

use crate::{error::Result, model::CalendarBmc, scraper::client::MainClient};

/// Fetches, scrapes and updates the heavy metal calendar for the current
/// year and saves it in the database.
pub async fn update_calendar(http_client: reqwest::Client) -> Result<()> {
    let client = MainClient::new(http_client);
    let year = OffsetDateTime::now_utc().year();

    let calendar1 = crate::scraper::metallum::scrape(&client, year).await?;
    let calendar2 = crate::scraper::wiki::scrape(&client, year).await?;
    let calendar = calendar1.merge(&calendar2);

    CalendarBmc::create_or_update(calendar).await?;
    CalendarBmc::update_bandcamp(&client).await?;

    Ok(())
}
