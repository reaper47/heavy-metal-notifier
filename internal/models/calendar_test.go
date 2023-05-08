package models_test

import (
	"github.com/reaper47/heavy-metal-notifier/internal/constants"
	"github.com/reaper47/heavy-metal-notifier/internal/models"
	"golang.org/x/exp/maps"
	"golang.org/x/exp/slices"
	"strconv"
	"testing"
	"time"
)

func TestCalendar_AddReleases(t *testing.T) {
	calendar := models.Calendar{}

	months := []time.Month{time.January, time.February, time.March, time.April, time.May, time.June, time.July, time.August, time.September, time.October, time.November, time.December}
	for i, month := range months {
		wantReleases := models.Releases{
			uint8(i): []models.Release{{Artist: strconv.Itoa(i + 1)}},
		}
		calendar.AddReleases(month, wantReleases)

		var gotReleases models.Releases
		switch month {
		case time.January:
			gotReleases = calendar.January
		case time.February:
			gotReleases = calendar.February
		case time.March:
			gotReleases = calendar.March
		case time.April:
			gotReleases = calendar.April
		case time.May:
			gotReleases = calendar.May
		case time.June:
			gotReleases = calendar.June
		case time.July:
			gotReleases = calendar.July
		case time.August:
			gotReleases = calendar.August
		case time.September:
			gotReleases = calendar.September
		case time.October:
			gotReleases = calendar.October
		case time.November:
			gotReleases = calendar.November
		case time.December:
			gotReleases = calendar.December
		}

		if !maps.EqualFunc(gotReleases, wantReleases, func(got []models.Release, want []models.Release) bool {
			return slices.EqualFunc(got, want, func(r models.Release, r2 models.Release) bool {
				return r.Artist == r2.Artist && r.Album == r2.Album
			})
		}) {
			t.Fatalf("want %+v but got %+v", wantReleases, gotReleases)
		}
	}
}

func TestCalendar_ReleasesOnDate(t *testing.T) {
	testcases := []struct {
		name string
		day  int
		want []models.Release
	}{
		{
			name: "day in calendar has releases",
			day:  25,
			want: []models.Release{{Artist: "Ensiferum", Album: "Into Battle"}},
		},
		{
			name: "day in calendar has no releases",
			day:  26,
			want: []models.Release{},
		},
	}
	for _, tc := range testcases {
		t.Run(tc.name, func(t *testing.T) {
			calendar := models.Calendar{
				December: models.Releases{
					uint8(25): []models.Release{{Artist: "Ensiferum", Album: "Into Battle"}},
				},
			}

			releases := calendar.ReleasesOnDate(time.December, tc.day)

			if len(releases) != len(tc.want) {
				t.Fatalf("expected no releases but got %+v", releases)
			}
		})
	}
}

func TestNewReleases(t *testing.T) {
	want := models.Releases{}

	got := models.NewReleases()

	if len(got) != len(want) {
		t.Fatalf("the new releases struct must be empty but got %+v", got)
	}
}

func TestRelease_URLs(t *testing.T) {
	testcases := []struct {
		name    string
		release models.Release
		want    []models.URL
	}{
		{
			name:    "Bandcamp and youtube exist",
			release: models.Release{Artist: "Abysmal Dawn", Album: "Nightmare Frontier (EP)"},
			want: []models.URL{
				{
					Name: constants.PlatformYouTube,
					URL:  "https://www.youtube.com/results?search_query=Abysmal+Dawn+Nightmare+Frontier+%28EP%29+full+album"},
				{
					Name: constants.PlatformBandcamp,
					URL:  "https://abysmaldawn.bandcamp.com",
				},
			},
		},
		{
			name:    "Bandcamp unavailable",
			release: models.Release{Artist: "Dark Funeral", Album: "We Are the Apocalypse"},
			want: []models.URL{
				{
					Name: constants.PlatformYouTube,
					URL:  "https://www.youtube.com/results?search_query=Dark+Funeral+We+Are+the+Apocalypse+full+album",
				},
			},
		},
		{
			name:    "Bandcamp and YouTube available #2",
			release: models.Release{Artist: "Vintersea", Album: "Woven into Ashes"},
			want: []models.URL{
				{
					Name: constants.PlatformYouTube,
					URL:  "https://www.youtube.com/results?search_query=Vintersea+Woven+into+Ashes+full+album",
				},
				{
					Name: constants.PlatformBandcamp,
					URL:  "https://vintersea.bandcamp.com",
				},
			},
		},
	}
	for _, tc := range testcases {
		t.Run(tc.name, func(t *testing.T) {
			tc.release.Links = tc.release.URLs()
			if !slices.EqualFunc(tc.release.Links, tc.want, func(u1 models.URL, u2 models.URL) bool {
				return u1.URL == u2.URL && u1.Name == u2.Name
			}) {
				t.Fatalf("got %v but want %v", tc.release.Links, tc.want)
			}
		})
	}
}
