use axum::{extract::Path, response::IntoResponse, routing::get, Router};
use reqwest::{header::CONTENT_TYPE, StatusCode};
use rss::{Channel, ChannelBuilder, Guid, Image, Item, ItemBuilder};
use time::{
    format_description::well_known::Rfc2822, util::days_in_year_month, Duration, OffsetDateTime,
};
use tracing::error;

use super::templates::calendar::{calendar, feeds};
use crate::{
    config::config,
    error::Result,
    model::{Artist, CalendarBmc, FeedBmc, Release},
};

pub fn routes_calendar() -> Router {
    Router::new()
        .route("/", get(calendar_handler))
        .route("/feed.xml", get(feed_handler))
        .route("/:year/:month/:day", get(releases_handler))
}

pub struct CalendarDay {
    pub day: u8,
    pub is_outside_month: bool,
}

async fn calendar_handler() -> impl IntoResponse {
    let now = OffsetDateTime::now_utc();
    let num_days_current_month = days_in_year_month(now.year(), now.month());

    let first_day_date = now.replace_day(1).unwrap_or(now);
    let last_day_date = now.replace_day(num_days_current_month).unwrap_or(now);

    let mut days: Vec<CalendarDay> = Vec::new();

    let offset_first_week = first_day_date.weekday() as u8 + 1;
    for i in 0..offset_first_week {
        let prev_date = first_day_date - Duration::days(i as i64);
        days.push(prev_date.date().day());
    }

    for i in 0..num_days_current_month {
        days.push(i + 1);
    }

    let weekday = last_day_date.weekday() as u8;
    let offset_last_week = match weekday {
        6 => 5,
        _ => weekday - 1,
    };
    for i in 0..offset_last_week {
        let next_date = last_day_date + Duration::days(i as i64);
        days.push(next_date.date().day());
    }

    calendar(days).into_response()
}

async fn feed_handler() -> impl IntoResponse {
    let now = OffsetDateTime::now_utc();
    let date_int =
        match format!("{}{:02}{:02}", now.year(), now.month() as u8, now.day()).parse::<i32>() {
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
    let base_url = config().BASE_URL.clone();

    let link_feed = format!("{}/calendar/feed.xml", base_url);
    let link_item: String = format!(
        "{}/calendar/{}/{}/{}",
        base_url,
        now.year(),
        now.month() as u8,
        now.day()
    );

    match FeedBmc::get(12) {
        Ok(feeds) => {
            let items = feeds
                .iter()
                .filter_map(|feed| {
                    Channel::read_from(feed.feed.as_bytes())
                        .ok()
                        .and_then(|channel| channel.items.first().cloned())
                })
                .collect::<Vec<_>>();

            let image = rss::ImageBuilder::default()
                .link(format!("{}/static/favicon.png", config().BASE_URL))
                .build();

            let channel = feeds
                .first()
                .and_then(|feed| {
                    if feed.date == date_int {
                        Some(build_channel_with_items(
                            &pub_date,
                            &link_feed,
                            image.clone(),
                            items,
                        ))
                    } else {
                        create_new_feed(
                            pub_date.clone(),
                            date.clone(),
                            date_int,
                            link_feed.clone(),
                            link_item.clone(),
                            image.clone(),
                        )
                        .ok()
                        .map(|channel| {
                            if let Some(item) = channel.items.first() {
                                let mut items_with_new = items.clone();
                                items_with_new.insert(0, item.clone());
                                build_channel_from_existing(channel, items)
                            } else {
                                build_channel_from_existing(channel, items)
                            }
                        })
                    }
                })
                .unwrap_or_else(|| {
                    create_new_feed(
                        pub_date.clone(),
                        date,
                        date_int,
                        link_feed.clone(),
                        link_item,
                        image.clone(),
                    )
                    .unwrap_or_else(|err| {
                        error!("Error creating new channel: {err}");
                        build_channel(pub_date, link_feed, image)
                    })
                });

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

fn create_new_feed(
    pub_date: String,
    date: String,
    date_int: i32,
    link_feed: impl Into<String>,
    link_item: impl Into<String>,
    image: Image,
) -> Result<Channel> {
    let releases = CalendarBmc::get().map_err(|err| {
        error!("Error fetching calendar: {}", err);
        err
    })?;

    let content = releases_to_html(releases);

    let channel = if content.is_empty() {
        build_channel(pub_date.clone(), link_feed.into(), image)
    } else {
        let mut guid = Guid::default();
        guid.set_value(date.to_string());

        let item = ItemBuilder::default()
            .title(date.clone())
            .pub_date(pub_date.clone())
            .content(content)
            .guid(guid)
            .link(Some(link_item.into()))
            .build();

        let channel = build_channel_with_items(&pub_date, link_feed.into(), image, vec![item]);

        if let Err(err) = FeedBmc::create(date_int, channel.to_string()) {
            error!("Error creating feed: {err}")
        }

        channel
    };

    Ok(channel)
}

fn build_channel(pub_date: String, link: String, image: Image) -> Channel {
    ChannelBuilder::default()
        .title("Heavy Metal Releases")
        .description("A feed for the latest heavy metal album releases.")
        .pub_date(pub_date.clone())
        .last_build_date(pub_date)
        .link(link)
        .image(image)
        .language("en-US".to_string())
        .build()
}

fn build_channel_with_items(
    pub_date: impl Into<String>,
    link: impl Into<String>,
    image: Image,
    items: Vec<Item>,
) -> Channel {
    let pub_date: String = pub_date.into();

    ChannelBuilder::default()
        .title("Heavy Metal Releases")
        .description("A feed for the latest heavy metal album releases.")
        .pub_date(pub_date.clone())
        .last_build_date(pub_date)
        .link(link)
        .image(image)
        .language("en-US".to_string())
        .items(items)
        .build()
}

fn build_channel_from_existing(channel: Channel, items: Vec<Item>) -> Channel {
    ChannelBuilder::default()
        .title(channel.title)
        .description(channel.description)
        .pub_date(channel.pub_date)
        .link(channel.link)
        .image(channel.image)
        .items(items)
        .build()
}

async fn releases_handler(Path((year, month, day)): Path<(u32, u8, u8)>) -> impl IntoResponse {
    match CalendarBmc::get_by_date(year, month, day) {
        Ok(releases) => {
            let date = format!("{year}-{month}-{day}");
            feeds(&date, releases).into_response()
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
