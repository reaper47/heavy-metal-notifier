package scraper

import (
	"github.com/PuerkitoBio/goquery"
	"metal-releases/internal/models"
	"strconv"
	"strings"
	"sync"
	"time"
)

func ScrapeMetalReleases(doc *goquery.Document) models.Calendar {
	months := []time.Month{
		time.January,
		time.February,
		time.March,
		time.April,
		time.May,
		time.June,
		time.July,
		time.August,
		time.September,
		time.October,
		time.November,
		time.December,
	}

	var calendar models.Calendar
	var wg sync.WaitGroup
	for _, month := range months {
		wg.Add(1)
		go func(month time.Month) {
			defer wg.Done()
			calendar.AddReleases(month, parseTable(doc, month))
		}(month)
	}
	wg.Wait()

	return calendar
}

func parseTable(doc *goquery.Document, month time.Month) models.Releases {
	releases := models.NewReleases()

	table := doc.Find("#table_" + month.String()).First()
	if table.Length() == 0 {
		return releases
	}

	rows := table.Find("tr")
	rows = rows.Slice(1, rows.Length())

	var day uint8
	rows.Each(func(i int, row *goquery.Selection) {
		data := row.Find("td")
		switch data.Length() {
		case 3:
			dayUint, err := strconv.ParseUint(strings.TrimSpace(data.First().Text()), 10, 8)
			if err != nil {
				panic(err)
			}
			day = uint8(dayUint)

			releases[day] = append(releases[day], models.Release{
				Artist: strings.TrimSpace(data.Eq(1).Text()),
				Album:  trimAlbumName(strings.TrimSpace(data.Eq(2).Text())),
			})
		case 2:
			releases[day] = append(releases[day], models.Release{
				Artist: strings.TrimSpace(data.Eq(0).Text()),
				Album:  trimAlbumName(strings.TrimSpace(data.Eq(1).Text())),
			})
		case 1:
			var artist string
			prevRow := rows.Slice(i-1, i).First().Find("td")
			switch prevRow.Length() {
			case 3:
				artist = strings.TrimSpace(prevRow.Eq(1).Text())
			case 2:
				artist = strings.TrimSpace(prevRow.Eq(0).Text())
			}

			var album string
			switch prevRow.Length() {
			case 3:
				album = data.First().Text()
			case 2:
				album = data.Text()
			}

			releases[day] = append(releases[day], models.Release{
				Artist: artist,
				Album:  trimAlbumName(strings.TrimSpace(album)),
			})
		}
	})

	return releases
}

func trimAlbumName(album string) string {
	if i := strings.Index(album, "["); i >= 0 {
		album = strings.TrimSpace(album[:i])
	}
	return album
}
