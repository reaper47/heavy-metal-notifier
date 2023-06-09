package templates

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

// DataError holds data on an error.
type DataError struct {
	Text string
}
