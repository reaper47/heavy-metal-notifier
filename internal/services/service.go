package services

import "metal-releases/internal/models"

type Service interface {
	// Register registers a new user to the service.
	Register(userEmail string) error

	// Unregister removes the user's access to the service.
	Unregister(userEmail string) error

	// Users fetches all users in the database.
	Users() ([]models.User, error)
}
