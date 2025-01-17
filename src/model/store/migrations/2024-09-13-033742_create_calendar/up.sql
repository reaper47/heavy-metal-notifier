CREATE TABLE artists (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL UNIQUE,
    genre TEXT,
    url_bandcamp TEXT,
    url_metallum TEXT
);

CREATE TABLE releases (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    year INTEGER NOT NULL,
    month INTEGER NOT NULL,
    day INTEGER NOT NULL,
    artist_id INTEGER NOT NULL REFERENCES artists (id) ON DELETE CASCADE,
    album VARCHAR  NOT NULL,
    release_type TEXT,
    url_youtube TEXT NOT NULL,
    url_metallum TEXT
);

CREATE TABLE feeds (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    date INTEGER NOT NULL,
    feed TEXT NOT NULL,
    custom_feed_id INTEGER NOT NULL REFERENCES custom_feeds (id) ON DELETE CASCADE
);

CREATE TABLE custom_feeds (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    bands TEXT NOT NULL,
    genres TEXT NOT NULL,
    UNIQUE (bands, genres)
);

INSERT INTO custom_feeds (id, bands, genres) VALUES (-1, '', '');
