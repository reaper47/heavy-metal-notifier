use diesel::prelude::*;

use diesel_migrations::MigrationHarness;
use diesel_migrations::{EmbeddedMigrations, embed_migrations};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("src/model/store/migrations");

/// Establishes a connection to the SQLite database.
///
/// # Panics
///
/// This function panics if the application of database migrations fails.
///
pub fn establish_connection() -> SqliteConnection {
    let mut conn = SqliteConnection::establish("./data/metal.db").unwrap_or_else(|_| {
        eprintln!("Error connecting to ./data/metal.db");
        std::process::exit(1);
    });

    conn.run_pending_migrations(MIGRATIONS)
        .expect("Migrations should have been applied");

    conn
}
