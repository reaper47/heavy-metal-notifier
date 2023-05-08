package models

import (
	"github.com/reaper47/heavy-metal-notifier/internal/constants"
	"net/http"
	"net/url"
	"strings"
	"time"
)

// Calendar is a struct that represents a monthly calendar with Releases for each month.
type Calendar struct {
	January   Releases
	February  Releases
	March     Releases
	April     Releases
	May       Releases
	June      Releases
	July      Releases
	August    Releases
	September Releases
	October   Releases
	November  Releases
	December  Releases
}

// AddReleases assigns Releases to the specified month in the Calendar struct.
func (c *Calendar) AddReleases(month time.Month, releases Releases) {
	switch month {
	case time.January:
		c.January = releases
	case time.February:
		c.February = releases
	case time.March:
		c.March = releases
	case time.April:
		c.April = releases
	case time.May:
		c.May = releases
	case time.June:
		c.June = releases
	case time.July:
		c.July = releases
	case time.August:
		c.August = releases
	case time.September:
		c.September = releases
	case time.October:
		c.October = releases
	case time.November:
		c.November = releases
	case time.December:
		c.December = releases
	}
}

// ReleasesOnDate returns a slice of Release structs for the specified month and day in the Calendar.
func (c *Calendar) ReleasesOnDate(month time.Month, day int) []Release {
	var releases Releases
	switch month {
	case time.January:
		releases = c.January
	case time.February:
		releases = c.February
	case time.March:
		releases = c.March
	case time.April:
		releases = c.April
	case time.May:
		releases = c.May
	case time.June:
		releases = c.June
	case time.July:
		releases = c.July
	case time.August:
		releases = c.August
	case time.September:
		releases = c.September
	case time.October:
		releases = c.October
	case time.November:
		releases = c.November
	case time.December:
		releases = c.December
	}

	if xr, ok := releases[uint8(day)]; ok {
		for i, r := range xr {
			xr[i].Links = r.URLs()
		}
		return xr
	}
	return []Release{}
}

// NewReleases creates and returns a Releases with an initial length of 0.
func NewReleases() Releases {
	return make(Releases, 0)
}

// Releases is a map that represents a collection of releases keyed by the day of the month.
type Releases map[uint8][]Release

// Release is a struct that represents a music release.
type Release struct {
	Artist string
	Album  string
	Links  []URL
}

// URLs generates a slice of URL structs containing various links for the given Release.
func (r *Release) URLs() []URL {
	urls := []URL{
		{
			Name: constants.PlatformYouTube,
			URL:  "https://www.youtube.com/results?search_query=" + url.QueryEscape(r.Artist+" "+r.Album+" full album"),
		},
	}

	artist := strings.ToLower(r.Artist)
	artist = strings.ReplaceAll(artist, " ", "")
	bandcampURL := "https://" + artist + ".bandcamp.com"
	res, err := http.Get(bandcampURL)
	if err != nil {
		return urls
	}
	defer res.Body.Close()

	if res.Request.URL.Path != "/signup" ||
		res.Request.Host == artist+".bandcamp.com" {
		urls = append(urls, URL{
			Name: constants.PlatformBandcamp,
			URL:  bandcampURL,
		})
	}

	return urls
}

// URL is a struct that represents a URL associated with a music release platform.
type URL struct {
	Name constants.Platform
	URL  string
}
