use diesel::prelude::*;

use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/model/store/migrations");

pub fn establish_connection(database_url: &str) -> SqliteConnection {
    let mut conn = SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    conn.run_pending_migrations(MIGRATIONS)
        .expect("migrations should have been applied");

    conn
}
