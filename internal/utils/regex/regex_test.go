package regex_test

import (
	"github.com/reaper47/heavy-metal-notifier/internal/utils/regex"
	"regexp"
	"testing"
)

func TestRegex(t *testing.T) {
	t.Parallel()

	testcases := []struct {
		name  string
		regex *regexp.Regexp
		in    string
		want  bool
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
		tc := tc
		t.Run(tc.name, func(t *testing.T) {
			t.Parallel()
			actual := tc.regex.MatchString(tc.in)
			if actual != tc.want {
				t.Fatalf("got %v but want %v for %s", actual, tc.want, tc.in)
			}
		})
	}
}
