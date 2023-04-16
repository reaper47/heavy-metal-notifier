package jobs

import (
	"fmt"
	"github.com/reaper47/heavy-metal-notifier/internal/app"
	"github.com/reaper47/heavy-metal-notifier/internal/constants"
	"github.com/reaper47/heavy-metal-notifier/internal/email"
	"github.com/reaper47/heavy-metal-notifier/internal/services"
	"github.com/reaper47/heavy-metal-notifier/internal/templates"
	"log"
)

func ScheduleCleanUsers(service services.Service) {
	if _, err := app.Scheduler.Every(2).MonthLastDay().Do(func() {
		if err := service.CleanDatabase(); err != nil {
			email.Send(app.Config.Email.From, constants.EmailErrorAdmin, templates.DataError{
				Text: fmt.Sprintf("ScheduleCleanUsers: cleaning users: %q", err),
			})
			return
		}
		log.Print("Cleaned database")
	}); err != nil {
		email.Send(app.Config.Email.From, constants.EmailErrorAdmin, templates.DataError{
			Text: fmt.Sprintf("ScheduleCleanUsers: error with scheduler: %q", err),
		})
	}
}
