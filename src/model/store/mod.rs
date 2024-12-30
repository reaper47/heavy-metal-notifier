use diesel::prelude::*;

use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/model/store/migrations");

/// Establishes a connection to the SQLite database.
///
/// # Panics
///
/// This function panics if the application of database migrations fails.
///
pub fn establish_connection(database_url: &str) -> SqliteConnection {
    let mut conn = SqliteConnection::establish(database_url).unwrap_or_else(|_| {
        eprintln!("Error connecting to {}", database_url);
        std::process::exit(1);
    });

    conn.run_pending_migrations(MIGRATIONS)
        .expect("Migrations should have been applied");

    conn
}
