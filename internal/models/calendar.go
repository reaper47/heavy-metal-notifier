package models

import "time"

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

func (c *Calendar) ReleasesOnDate(month time.Month, day int) []Release {
	//now := time.Now()
	dayUint8 := uint8(day)

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

	if xr, ok := releases[dayUint8]; ok {
		return xr
	}
	return []Release{}
}

func NewReleases() Releases {
	return make(Releases, 0)
}

type Releases map[uint8][]Release

type Release struct {
	Artist string
	Album  string
}
