package jobs

import (
	"encoding/base64"
	"fmt"
	"github.com/reaper47/heavy-metal-notifier/internal/app"
	"github.com/reaper47/heavy-metal-notifier/internal/services"
	"github.com/reaper47/heavy-metal-notifier/internal/templates"
	"log"
	"strings"
	"time"
)

// ScheduleCheckReleases schedules the cron job that checks whether there are new releases daily.
func ScheduleCheckReleases(repoService services.RepositoryService, emailService services.EmailService) {
	if _, err := app.Scheduler.Every(1).Day().At("01:00").Do(func() {
		now := time.Now()
		releases := app.Calendar.ReleasesOnDate(now.Month(), now.Day())
		if len(releases) > 0 {
			users, err := repoService.Users()
			if err != nil {
				emailService.Send(app.Config.Email.From, templates.EmailErrorAdmin, templates.DataError{
					Text: fmt.Sprintf("releases.go: error getting users: %q", err),
				})
				return
			}

			count := 0
			remaining, resetUnix, err := emailService.RateLimits()
			if err != nil {
				log.Println(err)
				return
			}

			for i, user := range users {
				if i == remaining {
					seconds := resetUnix - time.Now().Unix()
					if seconds > 0 {
						time.Sleep(time.Duration(seconds+1) * time.Second)

						count = 0
						remaining, resetUnix, err = emailService.RateLimits()
						if err != nil {
							log.Println(err)
							return
						}
					}
				}

				emailService.Send(user.Email, templates.EmailReleases, templates.EmailData{
					EmailBase64: base64.StdEncoding.EncodeToString([]byte(user.Email)),
					Name:        strings.Split(user.Email, "@")[0],
					Releases:    releases,
					URL:         app.Config.URL,
				})
				count++
			}
		}
	}); err != nil {
		emailService.Send(app.Config.Email.From, templates.EmailErrorAdmin, templates.DataError{
			Text: fmt.Sprintf("releases.go: error with scheduler: %q", err),
		})
	}
}
