package services

import "github.com/reaper47/heavy-metal-notifier/internal/models"

type Service interface {
	// Confirm confirms the account.
	Confirm(userEmail string) error

	// Register registers a new user to the service.
	Register(userEmail string) error

	// Unregister removes the user's access to the service.
	Unregister(userEmail string) error

	// Users fetches all users in the database.
	Users() ([]models.User, error)
}
