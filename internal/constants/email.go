package constants

// EmailTemplate represents the name of a .mjml email template.
type EmailTemplate string

const (
	EmailContact      EmailTemplate = "contact.mjml"
	EmailEndOfService EmailTemplate = "end-of-service.mjml"
	EmailErrorAdmin   EmailTemplate = "error-admin.mjml"
	EmailIntro        EmailTemplate = "intro.mjml"
	EmailReleases     EmailTemplate = "releases.mjml"
)

// String represents the email template as a string, being the file name.
func (e EmailTemplate) String() string {
	return string(e)
}

// Subject returns the subject of the email according to the type of email being sent.
func (e EmailTemplate) Subject() string {
	switch e {
	case EmailContact:
		return "Contact Form Request"
	case EmailEndOfService:
		return "End of Service"
	case EmailErrorAdmin:
		return "Heavy Metal Notifier Error"
	case EmailIntro:
		return "Welcome to Heavy Metal Releases Notifier"
	case EmailReleases:
		return "Latest Heavy Metal Releases"
	default:
		return ""
	}
}
