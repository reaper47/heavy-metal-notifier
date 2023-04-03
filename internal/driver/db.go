package driver

import (
	"context"
	"database/sql"
	"log"
	"time"

	_ "modernc.org/sqlite"
)

// ConnectSqlDB creates a connection to a SQLite database file.
func ConnectSqlDB(dsn string) *sql.DB {
	db, err := sql.Open("sqlite", dsn)
	if err != nil {
		log.Fatalln(err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()

	if err = db.PingContext(ctx); err != nil {
		log.Fatalf("Unable to ping the database: %s", err)
	}
	return db
}
