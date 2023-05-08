package jobs

import (
	"encoding/base64"
	"fmt"
	"github.com/reaper47/heavy-metal-notifier/internal/app"
	"github.com/reaper47/heavy-metal-notifier/internal/constants"
	"github.com/reaper47/heavy-metal-notifier/internal/email"
	"github.com/reaper47/heavy-metal-notifier/internal/services"
	"github.com/reaper47/heavy-metal-notifier/internal/templates"
	"strings"
	"time"
)

// ScheduleCheckReleases schedules the cron job that checks whether there are new releases daily.
func ScheduleCheckReleases(service services.Service) {
	if _, err := app.Scheduler.Every(1).Day().At("01:00").Do(func() {
		now := time.Now()
		releases := app.Calendar.ReleasesOnDate(now.Month(), now.Day())
		if len(releases) > 0 {
			users, err := service.Users()
			if err != nil {
				email.Send(app.Config.Email.From, constants.EmailErrorAdmin, templates.DataError{
					Text: fmt.Sprintf("releases.go: error getting users: %q", err),
				})
				return
			}

			for _, user := range users {
				email.Send(user.Email, constants.EmailReleases, templates.EmailData{
					EmailBase64: base64.StdEncoding.EncodeToString([]byte(user.Email)),
					Name:        strings.Split(user.Email, "@")[0],
					Releases:    releases,
					URL:         app.Config.URL,
				})
			}
		}
	}); err != nil {
		email.Send(app.Config.Email.From, constants.EmailErrorAdmin, templates.DataError{
			Text: fmt.Sprintf("releases.go: error with scheduler: %q", err),
		})
	}
}
