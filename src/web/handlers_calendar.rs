use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use reqwest::{header::CONTENT_TYPE, StatusCode};
use rss::{Channel, ChannelBuilder, Guid, Image, Item, ItemBuilder};
use time::{format_description::well_known::Rfc2822, OffsetDateTime};
use tracing::error;

use crate::{
    config::config,
    error::Result,
    model::{Artist, CalendarBmc, FeedBmc, Release},
};

pub fn routes_calendar() -> Router {
    Router::new()
        .route("/feed.xml", get(feed))
        .route("/:year/:month/:day", get(releases))
}

async fn feed() -> impl IntoResponse {
    let now = OffsetDateTime::now_utc();
    let date_int = match format!("{}{}{}", now.year(), now.month() as u8, now.day()).parse::<i32>()
    {
        Ok(n) => n,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not parse today's date.",
            )
                .into_response()
        }
    };

    let pub_date = now.format(&Rfc2822).unwrap_or_default();
    let date = format!("{} {}, {}", now.month(), now.day(), now.year());
    let link = format!("{}/{}/{}/{}", config().BASE_URL, now.year(), now.month(), now.day());

    match FeedBmc::get(12) {
        Ok(feeds) => {
            let mut items = feeds
                .iter()
                .map(|f| {
                    match Channel::read_from(f.feed.as_bytes()) {
                        Ok(channel) => channel.items.first().unwrap().clone(),
                        Err(err) => {
                            error!("Error reading channel item: {err}");
                            Item::default()
                        }
                    }
                })
                .collect::<Vec<_>>();

            let image = rss::ImageBuilder::default()
                .link(format!("{}/static/favicon.png", config().BASE_URL))
                .build();

            let channel = match feeds.first() {
                Some(feed) => {
                    if feed.date == date_int {
                        ChannelBuilder::default()
                            .title("Heavy Metal Releases")
                            .description("A feed for the latest heavy metal album releases.")
                            .pub_date(pub_date)
                            .link(link)
                            .image(image)
                            .items(items)
                            .build()
                    } else {
                        match create_new_feed(pub_date.clone(), date, date_int, link, image) {
                            Ok(channel) => {
                                if let Some(item) = channel.items.first() {
                                    items.insert(0, item.clone());
                                }

                                ChannelBuilder::default()
                                    .title(channel.title)
                                    .description(channel.description)
                                    .pub_date(channel.pub_date)
                                    .link(channel.link)
                                    .image(channel.image)
                                    .items(items)
                                    .build()
                            }
                            Err(err) => {
                                error!("Error creating new channel: {err}");
                                build_default_channel(pub_date)
                            }
                        }
                    }
                }
                None => match create_new_feed(pub_date.clone(), date, date_int, link, image) {
                    Ok(channel) => channel,
                    Err(err) => {
                        error!("Error creating new channel: {err}");
                        build_default_channel(pub_date)
                    }
                },
            };

            (
                [(CONTENT_TYPE, "text/xml;charset=UTF-8")],
                channel.to_string(),
            )
                .into_response()
        }
        Err(err) => {
            error!(
                "getting releases today {}: {err}",
                OffsetDateTime::now_utc()
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not fetch today's releases.",
            )
                .into_response()
        }
    }
}

fn create_new_feed(pub_date: String, date: String, date_int: i32, link: impl Into<String>, image: Image) -> Result<Channel> {
    match CalendarBmc::get() {
        Ok(releases) => {
            let content = releases_to_html(releases);

            let channel = if content.is_empty() {
                ChannelBuilder::default()
                    .title("Heavy Metal Releases")
                    .description("A feed for the latest heavy metal album releases.")
                    .pub_date(pub_date.clone())
                    .last_build_date(pub_date)
                    .language("en-US".to_string())
                    .link(link.into())
                    .image(image)
                    .build()
            } else {
                let mut guid = Guid::default();
                guid.set_value(date.to_string());

                let item = ItemBuilder::default()
                    .title(date.clone())
                    .pub_date(pub_date.clone())
                    .content(content)
                    .guid(guid)
                    .link(Some("".to_string()))
                    .build();

                let channel = ChannelBuilder::default()
                    .title("Heavy Metal Releases")
                    .description("A feed for the latest heavy metal album releases.")
                    .pub_date(pub_date)
                    .link("/calendar/feed.xml")
                    .item(item)
                    .build();

                let xml = channel.to_string();
                if let Err(err) = FeedBmc::create(date_int, xml.clone()) {
                    error!("Error creating feed: feed={xml}, error={err}")
                }

                channel
            };

            Ok(channel)
        }
        Err(err) => {
            error!("Error fetching calendar: {}", err);
            Err(err)
        }
    }
}

fn build_default_channel(pub_date: String) -> Channel {
    ChannelBuilder::default()
        .title("Heavy Metal Releases")
        .description("A feed for the latest heavy metal album releases.")
        .pub_date(pub_date.clone())
        .last_build_date(pub_date)
        .language("en-US".to_string())
        .link("/calendar/feed.xml")
        .build()
}

async fn releases(Path((year, month, day)): Path<(u32, u8, u8)>) -> impl IntoResponse {
    match CalendarBmc::get_by_date(year, month, day) {
        Ok(releases) => {
            let html = format!(r#"
                <!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>Releases {year}-{month}-{day}</title>
                </head>
                <body>
                    {}
                </body>
                </html>
            "#, releases_to_html(releases));

            ([(CONTENT_TYPE, "text/html;charset=UTF-8")], html).into_response()
        }
        Err(_) => (StatusCode::BAD_REQUEST, "No releases on this date.").into_response(),
    }
}

fn releases_to_html(releases: Vec<(Release, Artist)>) -> String {
    releases
        .iter()
        .fold("<ol>".to_string(), |mut acc, (release, artist)| {
            let html = release.to_html(artist);
            acc.push_str(&html);
            acc
        })
        + "</ol>"
}
