use axum::{
    extract::{Path, Query, State},
    http::HeaderMap,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::extract::Form;
use maud::Markup;
use reqwest::{header::CONTENT_TYPE, StatusCode};
use rss::{Channel, ChannelBuilder, Guid, Image, Item, ItemBuilder};
use serde::Deserialize;
use std::sync::Arc;
use time::{
    format_description::well_known::Rfc2822, util::days_in_month, Date, Duration, Month,
    OffsetDateTime, Time, UtcOffset,
};
use tracing::error;

use super::templates::calendar::{calendar, feeds, render_calendar};
use crate::model::FeedRepository;
use crate::{
    config::config,
    date_now,
    error::Result,
    model::{Artist, CalendarRepository, Feed, Release},
    web::AppState,
};

/// Defines the routes for the calendar feature of the web application.
pub fn routes_calendar() -> Router<AppState> {
    Router::new()
        .route("/", get(calendar_handler))
        .route("/:year/:month/:day/releases", get(calendar_month_handler))
        .route("/feed.xml", get(feed_handler).post(feed_post_handler))
        .route("/:year/:month/:day", get(releases_handler))
}

/// Represents a single day in a calendar, including its metadata.
pub struct CalendarDay {
    /// The day in the month.
    pub day: u8,

    /// Whether the calendar day is outside the month, e.g. November 31 is outside
    /// the month of November.
    pub is_outside_month: bool,

    /// The number of releases for the current calendar day.
    pub num_releases: Option<i64>,
}

async fn calendar_handler(State(state): State<AppState>, headers: HeaderMap) -> Markup {
    let now = date_now();
    let (days, releases) = calculate_calendar(state.calendar_repo, now);
    calendar(now, days, releases, headers)
}

async fn calendar_month_handler(
    State(state): State<AppState>,
    Path((year, month, day)): Path<(u32, String, u8)>,
) -> impl IntoResponse {
    let date = Date::from_calendar_date(
        year as i32,
        <Month as std::str::FromStr>::from_str(&month).unwrap_or(Month::January),
        day,
    )
    .unwrap_or(Date::from_calendar_date(2024, Month::October, 15).unwrap());

    let primitive_date_time = date.with_time(Time::MIDNIGHT);
    let date = primitive_date_time.assume_offset(UtcOffset::UTC);
    let (days, releases) = calculate_calendar(state.calendar_repo, date);

    render_calendar(date, days, releases)
}

fn calculate_calendar(
    repository: Arc<dyn CalendarRepository + Send + Sync>,
    date: OffsetDateTime,
) -> (Vec<CalendarDay>, Option<Vec<(Release, Artist)>>) {
    let num_days_current_month = days_in_month(date.month(), date.year());
    let mut days_in_prev_month = days_in_month(date.month().previous(), date.year());

    let first_day_date = date.replace_day(1).unwrap_or(date);
    let last_day_date = date.replace_day(num_days_current_month).unwrap_or(date);

    let mut days: Vec<CalendarDay> = Vec::new();

    // First week
    let offset_first_week = first_day_date.weekday() as u8 + 1;
    for _ in 0..offset_first_week {
        days.push(CalendarDay {
            day: days_in_prev_month,
            is_outside_month: true,
            num_releases: None,
        });

        days_in_prev_month -= 1;
    }
    days.reverse();

    for i in 0..num_days_current_month {
        days.push(CalendarDay {
            day: i + 1,
            is_outside_month: false,
            num_releases: repository.num_releases(date.year() as u32, date.month() as u8, i + 1),
        });
    }

    // Last week
    let weekday = last_day_date.weekday() as u8;
    const WEEKDAY_SATURDAY: u8 = 6;
    let offset_last_week = match weekday {
        WEEKDAY_SATURDAY => 5,
        _ => weekday.saturating_sub(1),
    };
    for i in 0..offset_last_week {
        let next_date = last_day_date + Duration::days((i as i64) + 1);
        days.push(CalendarDay {
            day: next_date.date().day(),
            is_outside_month: true,
            num_releases: None,
        });
    }

    (
        days,
        repository
            .get_by_date(date.year() as u32, date.month() as u8, date.day())
            .ok(),
    )
}

#[derive(Deserialize)]
struct FeedQuery {
    id: Option<i32>,
}

async fn feed_handler(
    State(state): State<AppState>,
    feed_query: Query<FeedQuery>,
) -> impl IntoResponse {
    let now = date_now();
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
    let base_url = &config().HOST_URL;

    let link_feed = format!("{}/calendar/feed.xml", base_url);
    let link_item: String = format!(
        "{}/calendar/{}/{}/{}",
        base_url,
        now.year(),
        now.month() as u8,
        now.day()
    );

    let custom_feed_id = feed_query.id.unwrap_or_else(|| -1);

    match state.feed_repo.get(12, custom_feed_id) {
        Ok(feeds) => (
            [(CONTENT_TYPE, "text/xml;charset=UTF-8")],
            create_channel(
                feeds,
                date,
                date_int,
                pub_date,
                link_feed,
                link_item,
                custom_feed_id,
                state.calendar_repo,
                state.feed_repo,
            )
            .to_string(),
        )
            .into_response(),
        Err(err) => {
            error!("getting releases today {}: {err}", date_now());
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Could not fetch today's releases.",
            )
                .into_response()
        }
    }
}

fn create_channel(
    feeds: Vec<Feed>,
    date: String,
    date_int: i32,
    pub_date: String,
    link_feed: String,
    link_item: String,
    custom_feed_id: i32,
    calendar_repo: Arc<dyn CalendarRepository + Send + Sync>,
    feed_repo: Arc<dyn FeedRepository + Send + Sync>,
) -> Channel {
    let items = feeds
        .iter()
        .filter_map(|feed| {
            Channel::read_from(feed.feed.as_bytes())
                .ok()
                .and_then(|channel| channel.items.first().cloned())
        })
        .collect::<Vec<_>>();

    let image_url = format!("{}/public/favicon.png", config().HOST_URL);
    let image = rss::ImageBuilder::default()
        .link(&image_url)
        .url(image_url)
        .build();

    feeds
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
                    custom_feed_id,
                    &calendar_repo,
                    &feed_repo,
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
                custom_feed_id,
                &calendar_repo,
                &feed_repo,
            )
            .unwrap_or_else(|err| {
                error!("Error creating new channel: {err}");
                build_channel(pub_date, link_feed, image)
            })
        })
}

fn create_new_feed(
    pub_date: String,
    date: String,
    date_int: i32,
    link_feed: impl Into<String>,
    link_item: impl Into<String>,
    image: Image,
    custom_feed_id: i32,
    calendar_repo: &Arc<dyn CalendarRepository + Send + Sync>,
    feed_repo: &Arc<dyn FeedRepository + Send + Sync>,
) -> Result<Channel> {
    let releases = calendar_repo.get().map_err(|err| {
        error!("Error fetching calendar: {}", err);
        err
    })?;

    let releases = if custom_feed_id > -1 {
        let custom_feed = feed_repo.get_custom_feed(custom_feed_id)?;
        let custom_feed_genres = custom_feed.genres;
        let custom_feed_bands = custom_feed.bands;

        if custom_feed_genres == "none" {
            releases
                .into_iter()
                .filter(|(_releases, artist)| {
                    custom_feed_bands.contains(&artist.name.to_lowercase())
                })
                .collect::<Vec<_>>()
        } else if custom_feed_bands == "none" {
            releases
                .into_iter()
                .filter(|(_releases, artist)| {
                    let mut is_genre_in_want = artist.genre.is_some();

                    if let Some(ref genre) = artist.genre {
                        is_genre_in_want = contains_any_keywords(
                            &genre.to_lowercase().replace(" metal", ""),
                            &custom_feed_genres,
                        );
                    }

                    is_genre_in_want
                })
                .collect::<Vec<_>>()
        } else {
            releases
                .into_iter()
                .filter(|(_releases, artist)| {
                    let mut is_genre_in_want = artist.genre.is_some();

                    if let Some(ref genre) = artist.genre {
                        is_genre_in_want = contains_any_keywords(
                            &genre.to_lowercase().replace(" metal", ""),
                            &custom_feed_genres,
                        );
                    }

                    is_genre_in_want || custom_feed_bands.contains(&artist.name.to_lowercase())
                })
                .collect::<Vec<_>>()
        }
    } else {
        releases
    };

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

        if let Err(err) = feed_repo.create(date_int, &channel.to_string(), custom_feed_id) {
            error!("Error creating feed: {err}")
        }

        channel
    };

    Ok(channel)
}

fn contains_any_keywords(genre: &str, keywords: &str) -> bool {
    let normalized_genre = genre.to_lowercase();
    let genre_words: Vec<&str> = normalized_genre
        .split(|c: char| c.is_whitespace() || c == ',' || c == ';')
        .collect();

    keywords
        .to_lowercase()
        .split('@')
        .any(|keyword| genre_words.iter().any(|&word| word.contains(keyword)))
}

fn build_channel(pub_date: String, link: String, image: Image) -> Channel {
    ChannelBuilder::default()
        .title("Heavy Metal Releases")
        .description("A feed for the latest heavy metal album releases.")
        .pub_date(pub_date.clone())
        .last_build_date(pub_date)
        .link(link)
        .image(image)
        .language(String::from("en-US"))
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
        .language(String::from("en-US"))
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

#[derive(Deserialize)]
struct GenerateFeedForm {
    #[serde(default)]
    bands: Vec<String>,
    #[serde(default)]
    genres: Vec<String>,
}

async fn feed_post_handler(
    State(state): State<AppState>,
    Form(form): Form<GenerateFeedForm>,
) -> impl IntoResponse {
    match state
        .feed_repo
        .get_or_create_custom_feed(form.bands, form.genres)
    {
        None => Redirect::to("/calendar/feed.xml").into_response(),
        Some(id) => {
            let url = &format!("{}/calendar/feed.xml?id={id}", config().HOST_URL);
            let input = format!("<input id=\"custom_link\" hx-swap-oob=\"true\" readonly type=\"text\" placeholder=\"Your custom link\" class=\"input input-bordered w-full mt-1\" value=\"{url}\">");
            (StatusCode::OK, input).into_response()
        }
    }
}

async fn releases_handler(
    State(state): State<AppState>,
    Path((year, month, day)): Path<(u32, u8, u8)>,
) -> impl IntoResponse {
    match state.calendar_repo.get_by_date(year, month, day) {
        Ok(releases) => {
            let date = format!("{year}-{month}-{day}");
            feeds(&date, releases).into_response()
        }
        Err(_) => (StatusCode::BAD_REQUEST, "No releases on this date.").into_response(),
    }
}

fn releases_to_html(releases: Vec<(Release, Artist)>) -> String {
    releases.iter().fold(
        String::from("<ol id=\"feeds__container\" class=\"list-disc\">"),
        |mut acc, (release, artist)| {
            let html = release.to_html(artist);
            acc.push_str(&html);
            acc
        },
    ) + "</ol>"
}
