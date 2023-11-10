package services

import (
	"errors"
	"github.com/reaper47/heavy-metal-notifier/internal/app"
	"github.com/reaper47/heavy-metal-notifier/internal/templates"
	"github.com/sendgrid/sendgrid-go"
	"github.com/sendgrid/sendgrid-go/helpers/mail"
	"jaytaylor.com/html2text"
	"log"
	"strconv"
)

// NewEmailService creates a new Email service.
func NewEmailService() *Email {
	return &Email{}
}

// EmailService is the interface that describes the methods required for the email client.
type EmailService interface {
	// RateLimits gets the SendGrid API's remaining and reset rate limits.
	RateLimits() (remaining int, resetUnix int64, err error)

	// Send sends an email using the SendGrid API.
	Send(to string, template templates.EmailTemplate, data any)
}

type request struct {
	from    string
	to      string
	subject string
	body    string
}

// Email is the entity that manages the email client.
type Email struct{}

func (e *Email) RateLimits() (remaining int, resetUnix int64, err error) {
	req := sendgrid.GetRequest(app.Config.Email.SendGridAPIKey, "/v3/templates", "https://api.sendgrid.com")

	res, err := sendgrid.API(req)
	if err != nil {
		return -1, -1, err
	}

	xs, ok := res.Headers["X-Ratelimit-Remaining"]
	if !ok {
		return -1, -1, errors.New("cannot find the X-RateLimit-Remaining header")
	}

	rem, err := strconv.Atoi(xs[0])
	if err != nil {
		return -1, -1, err
	}

	xs, ok = res.Headers["X-Ratelimit-Reset"]
	if !ok {
		return -1, -1, errors.New("cannot find the X-RateLimit-Reset header")
	}

	reset, err := strconv.ParseInt(xs[0], 10, 64)
	if err != nil {
		return -1, -1, err
	}

	return rem, reset, nil
}

// Send sends an email of the given template to a recipient.
func (e *Email) Send(to string, template templates.EmailTemplate, data any) {
	go func() {
		r := &request{
			from:    app.Config.Email.From,
			to:      to,
			subject: template.Subject(),
		}

		buf := templates.RenderEmail(template.String(), data)
		r.body = buf

		if err := r.sendMail(); err != nil {
			log.Printf("error sending %s email to %s: %q", template, to, err)
		}
	}()
}

func (r *request) sendMail() error {
	text, err := html2text.FromString(r.body, html2text.Options{TextOnly: false})
	if err != nil {
		return err
	}

	client := sendgrid.NewSendClient(app.Config.Email.SendGridAPIKey)

	from := mail.NewEmail("Heavy Metal Releases", r.from)
	to := mail.NewEmail(r.subject, r.to)
	_, err = client.Send(mail.NewSingleEmail(from, r.subject, to, text, r.body))
	return err
}
