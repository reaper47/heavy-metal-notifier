package services

import (
	"context"
	"database/sql"
	"embed"
	"github.com/pressly/goose/v3"
	"github.com/reaper47/heavy-metal-notifier/internal/constants"
	"github.com/reaper47/heavy-metal-notifier/internal/driver"
	"github.com/reaper47/heavy-metal-notifier/internal/models"
	"github.com/reaper47/heavy-metal-notifier/internal/services/statements"
	"log"
	"os"
	"path/filepath"
	"sync"
	"time"
)

//go:embed migrations/*.sql
var embedMigrations embed.FS

// NewSQLiteService creates an SQLite service.
func NewSQLiteService() *SQLiteService {
	exe, err := os.Executable()
	if err != nil {
		log.Fatal(err)
	}

	path := filepath.Join(filepath.Dir(exe), constants.DBName)
	dsnURI := "file:" + path + "?" +
		"_pragma=foreign_keys(1)" +
		"&_pragma=journal_mode(wal)" +
		"&_pragma=synchronous(normal)" +
		"&_pragma=temp_store(memory)"

	db := driver.ConnectSqlDB(dsnURI)

	goose.SetBaseFS(embedMigrations)

	if err := goose.SetDialect("sqlite"); err != nil {
		log.Fatal(err)
	}

	if err := goose.Up(db, "migrations"); err != nil {
		log.Fatal(err)
	}

	return &SQLiteService{
		DB:    db,
		Mutex: &sync.Mutex{},
	}
}

// SQLiteService represents the Service implemented with SQLite.
type SQLiteService struct {
	DB    *sql.DB
	Mutex *sync.Mutex
}

func (s *SQLiteService) Register(userEmail string) error {
	s.Mutex.Lock()
	defer s.Mutex.Unlock()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	_, err := s.DB.ExecContext(ctx, statements.InsertUser, userEmail)
	return err
}

func (s *SQLiteService) Unregister(userEmail string) error {
	s.Mutex.Lock()
	defer s.Mutex.Unlock()

	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	_, err := s.DB.ExecContext(ctx, statements.DeleteUser, userEmail)
	return err
}

func (s *SQLiteService) Users() ([]models.User, error) {
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	rows, err := s.DB.QueryContext(ctx, statements.SelectUsers)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	var users []models.User
	for rows.Next() {
		var email string
		if err := rows.Scan(&email); err != nil {
			return nil, err
		}
		users = append(users, models.User{Email: email})
	}

	return users, nil
}
