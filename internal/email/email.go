package email

import (
	"github.com/sendgrid/sendgrid-go"
	"github.com/sendgrid/sendgrid-go/helpers/mail"
	"jaytaylor.com/html2text"
	"log"
	"metal-releases/internal/app"
	"metal-releases/internal/constants"
	"metal-releases/internal/templates"
)

type request struct {
	from    string
	to      string
	subject string
	body    string
}

// Send sends an email of the given template to a recipient.
func Send(to string, template constants.EmailTemplate, data any) {
	go func() {
		r := &request{
			from:    app.Config.Email.From,
			to:      to,
			subject: template.Subject(),
		}

		if err := r.send(template.String(), data); err != nil {
			log.Printf("error sending %s email to %s: %q", template, to, err)
		}
	}()
}

func (r *request) send(template string, data any) error {
	if err := r.parseTemplate(template, data); err != nil {
		return err
	}
	return r.sendMail()
}

func (r *request) parseTemplate(fileName string, data interface{}) error {
	buf, err := templates.RenderEmail(fileName, data)
	if err != nil {
		return err
	}

	r.body = buf
	return nil
}

func (r *request) sendMail() error {
	text, err := html2text.FromString(r.body, html2text.Options{TextOnly: true})
	if err != nil {
		return err
	}

	client := sendgrid.NewSendClient(app.Config.Email.SendGridAPIKey)

	from := mail.NewEmail(r.subject, r.from)
	to := mail.NewEmail(r.subject, r.to)
	_, err = client.Send(mail.NewSingleEmail(from, r.subject, to, text, r.body))
	return err
}
