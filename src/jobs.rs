//! The `jobs` module implements functions that are meant to be run periodically.

use crate::{date_now, error::Result, model::CalendarRepository, scraper::client::MainClient};

/// Fetches, scrapes and updates the heavy metal calendar for the current
/// year and saves it in the database.
pub async fn update_calendar<R>(calendar_repo: R) -> Result<()>
where
    R: CalendarRepository,
{
    let http_client = reqwest::Client::new();
    let client = MainClient::new(http_client);
    let year = date_now().year();

    let calendar1 = crate::scraper::metallum::scrape(&client, year).await?;
    let calendar2 = crate::scraper::wiki::scrape(&client, year).await?;
    let calendar = calendar1.merge(&calendar2);

    calendar_repo.create_or_update(calendar).await?;
    calendar_repo.update_bandcamp(&client).await?;

    Ok(())
}
