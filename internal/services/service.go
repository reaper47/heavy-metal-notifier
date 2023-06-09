package services

import "github.com/reaper47/heavy-metal-notifier/internal/models"

// RepositoryService is the interface that describes the methods required for managing the main data store.
type RepositoryService interface {
	// CleanDatabase cleans the database. Unconfirmed users are wiped.
	CleanDatabase() error

	// Confirm confirms the account.
	Confirm(userEmail string) error

	// Register registers a new user to the service.
	Register(userEmail string) error

	// Unregister removes the user's access to the service.
	Unregister(userEmail string) error

	// Users fetches all users in the database.
	Users() ([]models.User, error)
}
