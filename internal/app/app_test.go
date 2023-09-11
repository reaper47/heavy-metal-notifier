package app_test

import (
	"errors"
	"github.com/reaper47/heavy-metal-notifier/internal/app"
	"testing"
)

func TestConfigFile_Valid(t *testing.T) {
	testcases := []struct {
		name   string
		config app.ConfigFile
		want   error
	}{
		{
			name: "missing email from",
			config: app.ConfigFile{
				Email: app.ConfigEmail{
					MaxNumberUsers: 100,
					SendGridAPIKey: "sgv",
				},
				Port: 8000,
				URL:  "localhost",
			},
			want: errors.New("configuration file has invalid email.from"),
		},
		{
			name: "invalid email from",
			config: app.ConfigFile{
				Email: app.ConfigEmail{
					From:           "test.com",
					MaxNumberUsers: 100,
					SendGridAPIKey: "sgv",
				},
				Port: 8000,
				URL:  "localhost",
			},
			want: errors.New("configuration file has invalid email.from"),
		},
		{
			name: "missing email max number of users",
			config: app.ConfigFile{
				Email: app.ConfigEmail{
					From:           "test@gmail.com",
					SendGridAPIKey: "sgv",
				},
				Port: 8000,
				URL:  "localhost",
			},
			want: errors.New("configuration file missing email.maxNumberUsers"),
		},
		{
			name: "missing email sendgrid API key",
			config: app.ConfigFile{
				Email: app.ConfigEmail{
					From:           "test@gmail.com",
					MaxNumberUsers: 100,
				},
				Port: 8000,
				URL:  "localhost",
			},
			want: errors.New("configuration file missing email.sendGridAPIKey"),
		},
		{
			name: "invalid port",
			config: app.ConfigFile{
				Email: app.ConfigEmail{
					From:           "test@gmail.com",
					MaxNumberUsers: 100,
					SendGridAPIKey: "sgv",
				},
				Port: 24,
				URL:  "localhost",
			},
			want: errors.New("configuration file has invalid port"),
		},
		{
			name: "missing url",
			config: app.ConfigFile{
				Email: app.ConfigEmail{
					From:           "test@gmail.com",
					MaxNumberUsers: 100,
					SendGridAPIKey: "sgv",
				},
				Port: 1024,
			},
			want: errors.New("configuration file missing url"),
		},
	}
	for _, tc := range testcases {
		t.Run(tc.name, func(t *testing.T) {
			isValid, gotErr := tc.config.Valid()
			if isValid {
				t.Fatal("config file is invalid but was valid")
			}

			if gotErr.Error() != tc.want.Error() {
				t.Fatalf("got error %q but want %q", gotErr, tc.want)
			}
		})
	}

	t.Run("file is valid", func(t *testing.T) {
		c := app.ConfigFile{
			Email: app.ConfigEmail{
				From:           "test@gmail.com",
				MaxNumberUsers: 100,
				SendGridAPIKey: "sgv",
			},
			Port: 1024,
			URL:  "localhost",
		}

		isValid, gotErr := c.Valid()
		if !isValid {
			t.Fatal("config file was invalid when it should have been")
		}

		if gotErr != nil {
			t.Fatalf("got err %q when expected nil", gotErr)
		}
	})
}
