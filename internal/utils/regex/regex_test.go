package regex_test

import (
	"golang.org/x/exp/slices"
	"metal-releases/internal/utils/regex"
	"regexp"
	"testing"
)

func TestRegex(t *testing.T) {
	testcases := []struct {
		name          string
		regex         *regexp.Regexp
		in            string
		want          bool
		wantedMatches []string
	}{
		{
			name:  "email is valid",
			regex: regex.Email,
			in:    "xyz@gmail.com",
			want:  true,
		},
		{
			name:  "email is invalid 1",
			regex: regex.Email,
			in:    "zebra.com",
			want:  false,
		},
		{
			name:  "email is invalid 2",
			regex: regex.Email,
			in:    "@gmail.com",
			want:  false,
		},
	}
	for _, tc := range testcases {
		t.Run(tc.name, func(t *testing.T) {
			if len(tc.wantedMatches) > 0 {
				actual := tc.regex.FindAllString(tc.in, -1)
				for _, v := range tc.wantedMatches {
					if slices.Index(actual, v) == -1 {
						t.FailNow()
					}
				}
			} else {
				actual := tc.regex.MatchString(tc.in)
				if actual != tc.want {
					t.Fatalf("got %v but want %v for %s", actual, tc.want, tc.in)
				}
			}
		})
	}
}
