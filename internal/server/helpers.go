package server

import (
	"fmt"
	"github.com/reaper47/heavy-metal-notifier/internal/app"
	"github.com/reaper47/heavy-metal-notifier/internal/templates"
)

func sendErrorAdminEmail(sendFunc func(to string, template templates.EmailTemplate, data any), errFuncName string, err error) {
	sendFunc(app.Config.Email.From, templates.EmailErrorAdmin, templates.EmailData{
		Text: fmt.Sprintf("error in %s: %q", errFuncName, err),
	})
}
