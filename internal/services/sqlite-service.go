package services

import (
	"context"
	"database/sql"
	"embed"
	"github.com/pressly/goose/v3"
	"github.com/reaper47/heavy-metal-notifier/internal/constants"
	"github.com/reaper47/heavy-metal-notifier/internal/models"
	"github.com/reaper47/heavy-metal-notifier/internal/services/statements"
	"log"
	_ "modernc.org/sqlite"
	"os"
	"path/filepath"
	"sync"
	"time"
)

//go:embed migrations/*.sql
var embedMigrations embed.FS

const (
	shortContextDeadline = 10 * time.Second
)

// NewSQLiteService creates an SQLite service.
func NewSQLiteService() *SQLiteService {
	exe, err := os.Executable()
	if err != nil {
		log.Fatal(err)
	}

	path := filepath.Join(filepath.Dir(exe), constants.DBName)
	dsn := "file:" + path + "?" +
		"_pragma=foreign_keys(1)" +
		"&_pragma=journal_mode(wal)" +
		"&_pragma=synchronous(normal)" +
		"&_pragma=temp_store(memory)"

	db, err := sql.Open("sqlite", dsn)
	if err != nil {
		log.Fatalln(err)
	}

	ctx, cancel := context.WithTimeout(context.Background(), 3*time.Second)
	defer cancel()

	if err = db.PingContext(ctx); err != nil {
		log.Fatalf("Unable to ping the database: %s", err)
	}

	goose.SetBaseFS(embedMigrations)

	err = goose.SetDialect("sqlite")
	if err != nil {
		log.Fatal(err)
	}

	err = goose.Up(db, "migrations")
	if err != nil {
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

func (s *SQLiteService) CleanDatabase() error {
	s.Mutex.Lock()
	defer s.Mutex.Unlock()

	ctx, cancel := context.WithTimeout(context.Background(), shortContextDeadline)
	defer cancel()

	_, err := s.DB.ExecContext(ctx, statements.DeleteUnconfirmedUsers)
	return err
}

func (s *SQLiteService) Confirm(userEmail string) error {
	s.Mutex.Lock()
	defer s.Mutex.Unlock()

	ctx, cancel := context.WithTimeout(context.Background(), shortContextDeadline)
	defer cancel()

	_, err := s.DB.ExecContext(ctx, statements.ConfirmAccount, userEmail)
	return err
}

func (s *SQLiteService) Register(userEmail string) error {
	s.Mutex.Lock()
	defer s.Mutex.Unlock()

	ctx, cancel := context.WithTimeout(context.Background(), shortContextDeadline)
	defer cancel()

	_, err := s.DB.ExecContext(ctx, statements.InsertUser, userEmail)
	return err
}

func (s *SQLiteService) Unregister(userEmail string) error {
	s.Mutex.Lock()
	defer s.Mutex.Unlock()

	ctx, cancel := context.WithTimeout(context.Background(), shortContextDeadline)
	defer cancel()

	_, err := s.DB.ExecContext(ctx, statements.DeleteUser, userEmail)
	return err
}

func (s *SQLiteService) Users() ([]models.User, error) {
	ctx, cancel := context.WithTimeout(context.Background(), shortContextDeadline)
	defer cancel()

	rows, err := s.DB.QueryContext(ctx, statements.SelectUsers)
	if err != nil {
		return nil, err
	}
	defer func() {
		_ = rows.Close()
	}()

	if rows.Err() != nil {
		return nil, err
	}

	var users []models.User
	for rows.Next() {
		var email string
		err = rows.Scan(&email)
		if err != nil {
			return nil, err
		}
		users = append(users, models.User{Email: email})
	}

	return users, nil
}
