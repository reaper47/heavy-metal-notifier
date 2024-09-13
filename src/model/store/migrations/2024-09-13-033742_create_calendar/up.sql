CREATE TABLE artists (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL
);

CREATE TABLE releases (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    year INTEGER NOT NULL,
    month INTEGER NOT NULL,
    day INTEGER NOT NULL,
    artist_id INTEGER NOT NULL REFERENCES artists (id) ON DELETE CASCADE,
    album VARCHAR  NOT NULL
);

CREATE TABLE links (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    artist_id INTEGER NOT NULL REFERENCES artists (id) ON DELETE CASCADE,
    url_youtube TEXT NOT NULL,
    url_bandcamp TEXT
);
