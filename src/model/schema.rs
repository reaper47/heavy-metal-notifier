// @generated automatically by Diesel CLI.

diesel::table! {
    artists (id) {
        id -> Integer,
        name -> Text,
        genre -> Nullable<Text>,
        url_bandcamp -> Nullable<Text>,
        url_metallum -> Nullable<Text>,
    }
}

diesel::table! {
    custom_feeds (id) {
        id -> Integer,
        bands -> Text,
        genres -> Text,
    }
}

diesel::table! {
    feeds (id) {
        id -> Integer,
        date -> Integer,
        feed -> Text,
        custom_feed_id -> Integer,
    }
}

diesel::table! {
    releases (id) {
        id -> Integer,
        year -> Integer,
        month -> Integer,
        day -> Integer,
        artist_id -> Integer,
        album -> Text,
        release_type -> Nullable<Text>,
        url_youtube -> Text,
        url_metallum -> Nullable<Text>,
    }
}

diesel::joinable!(feeds -> custom_feeds (custom_feed_id));
diesel::joinable!(releases -> artists (artist_id));

diesel::allow_tables_to_appear_in_same_query!(artists, custom_feeds, feeds, releases,);
