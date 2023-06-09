package jobs

import (
	"fmt"
	"github.com/reaper47/heavy-metal-notifier/internal/app"
	"github.com/reaper47/heavy-metal-notifier/internal/services"
	"github.com/reaper47/heavy-metal-notifier/internal/templates"
	"log"
)

func ScheduleCleanUsers(repoService services.RepositoryService, emailService services.EmailService) {
	if _, err := app.Scheduler.Every(2).MonthLastDay().Do(func() {
		if err := repoService.CleanDatabase(); err != nil {
			emailService.Send(app.Config.Email.From, templates.EmailErrorAdmin, templates.DataError{
				Text: fmt.Sprintf("ScheduleCleanUsers: cleaning users: %q", err),
			})
			return
		}
		log.Print("Cleaned database")
	}); err != nil {
		emailService.Send(app.Config.Email.From, templates.EmailErrorAdmin, templates.DataError{
			Text: fmt.Sprintf("ScheduleCleanUsers: error with scheduler: %q", err),
		})
	}
}
