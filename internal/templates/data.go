package templates

import "metal-releases/internal/models"

// Data holds general template data.
type Data struct {
	ShowNav       bool
	IsAboutPage   bool
	IsContactPage bool
	IsMessageSent bool
	IsHomePage    bool

	PageTitle    string
	ContentTitle string
	Content      string
	EmailBase64  string
}

// EmailData holds data for email templates.
type EmailData struct {
	From    string
	Message string
	URL     string

	EmailBase64 string
	Name        string
	Releases    []models.Release
}

// DataError holds data on an error.
type DataError struct {
	Text string
}
