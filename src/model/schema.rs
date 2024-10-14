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
    feeds (id) {
        id -> Integer,
        date -> Integer,
        feed -> Text,
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

diesel::joinable!(releases -> artists (artist_id));

diesel::allow_tables_to_appear_in_same_query!(artists, feeds, releases,);
