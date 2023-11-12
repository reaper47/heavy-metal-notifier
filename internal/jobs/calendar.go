package jobs

import (
	"fmt"
	"github.com/PuerkitoBio/goquery"
	"github.com/reaper47/heavy-metal-notifier/internal/app"
	"github.com/reaper47/heavy-metal-notifier/internal/scraper"
	"log"
	"net/http"
	"time"
)

// ScheduleFetchCalendar starts a weekly job to fetch the metal releases of the current year.
func ScheduleFetchCalendar() {
	fetchCalendar()
	if _, err := app.Scheduler.Every(1).Week().Friday().Do(func() {
		fetchCalendar()
	}); err != nil {
		panic(err)
	}
}

func fetchCalendar() {
	now := time.Now()

	res, err := http.Get(fmt.Sprintf("https://en.wikipedia.org/wiki/%d_in_heavy_metal_music", now.Year()))
	if err != nil {
		panic(err)
	}
	defer func() {
		_ = res.Body.Close()
	}()

	doc, err := goquery.NewDocumentFromReader(res.Body)
	if err != nil {
		panic(err)
	}

	app.Calendar = scraper.ScrapeMetalReleases(doc)
	log.Print("Updated calendar")
}
