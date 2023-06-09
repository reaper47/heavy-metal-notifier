package templates_test

import (
	"github.com/reaper47/heavy-metal-notifier/internal/templates"
	"testing"
)

var emailTemplates = []templates.EmailTemplate{
	templates.EmailContact,
	templates.EmailEndOfService,
	templates.EmailErrorAdmin,
	templates.EmailIntro,
	templates.EmailReleases,
}

func TestEmailTemplate_String(t *testing.T) {
	want := []string{
		"contact.mjml",
		"end-of-service.mjml",
		"error-admin.mjml",
		"intro.mjml",
		"releases.mjml",
	}
	for i, template := range emailTemplates {
		if got := template.String(); got != want[i] {
			t.Fatalf("got %q but want %q", got, want[i])
		}
	}
}

func TestEmailTemplate_Subject(t *testing.T) {
	want := []string{
		"Contact Form Request",
		"End of Service",
		"Heavy Metal Notifier Error",
		"Welcome to Heavy Metal Releases Notifier",
		"Latest Heavy Metal Releases",
	}
	for i, template := range emailTemplates {
		if got := template.Subject(); got != want[i] {
			t.Fatalf("got %q but want %q", got, want[i])
		}
	}
}
