package constants

import "testing"

func TestEmailTemplate_Subject(t *testing.T) {
	testcases := []struct {
		in   EmailTemplate
		want string
	}{
		{in: EmailContact, want: "Contact Form Request"},
		{in: EmailEndOfService, want: "End of Service"},
		{in: EmailErrorAdmin, want: "Heavy Metal Notifier Error"},
		{in: EmailIntro, want: "Welcome to Heavy Metal Releases Notifier"},
		{in: EmailReleases, want: "Latest Heavy Metal Releases"},
	}
	for _, tc := range testcases {
		t.Run(tc.in.String(), func(t *testing.T) {
			if actual := tc.in.Subject(); actual != tc.want {
				t.Errorf("got %q but want %q", actual, tc.want)
			}
		})
	}
}
