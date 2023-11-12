package app

import (
	"encoding/json"
	"errors"
	"github.com/go-co-op/gocron"
	"github.com/reaper47/heavy-metal-notifier/internal/constants"
	"github.com/reaper47/heavy-metal-notifier/internal/models"
	"github.com/reaper47/heavy-metal-notifier/internal/utils/regex"
	"log"
	"os"
	"path/filepath"
	"time"
)

var (
	Calendar  models.Calendar
	Config    ConfigFile
	Scheduler *gocron.Scheduler
)

// ConfigFile holds the contents of config.json.
type ConfigFile struct {
	Email ConfigEmail `json:"email"`
	Port  int         `json:"port"`
	URL   string      `json:"url"`
}

// Valid verifies whether the configuration is correct.
func (c *ConfigFile) Valid() (bool, error) {
	isValid, err := c.Email.valid()
	if !isValid {
		return isValid, err
	}

	if c.Port < 1024 {
		return false, errors.New("configuration file has invalid port")
	}

	if c.URL == "" {
		return false, errors.New("configuration file missing url")
	}

	return true, nil
}

// ConfigEmail holds email configuration variables.
type ConfigEmail struct {
	From           string `json:"from"`
	MaxNumberUsers int    `json:"maxNumberUsers"`
	SendGridAPIKey string `json:"sendGridApiKey"`
}

func (c *ConfigEmail) valid() (bool, error) {
	if !regex.Email.MatchString(c.From) {
		return false, errors.New("configuration file has invalid email.from")
	}

	if c.MaxNumberUsers == 0 {
		return false, errors.New("configuration file missing email.maxNumberUsers")
	}

	if c.SendGridAPIKey == "" {
		return false, errors.New("configuration file missing email.sendGridAPIKey")
	}

	return true, nil
}

// Init initializes the application by reading the config file and starting chron jobs.
func Init() {
	exe, err := os.Executable()
	if err != nil {
		log.Fatal(err)
	}

	xb, err := os.ReadFile(filepath.Join(filepath.Dir(exe), constants.ConfigFile))
	if err != nil {
		log.Fatal(err)
	}

	err = json.Unmarshal(xb, &Config)
	if err != nil {
		log.Fatal(err)
	}

	isValid, err := Config.Valid()
	if !isValid {
		log.Fatal(err.Error())
	}

	Scheduler = gocron.NewScheduler(time.Local)
	Scheduler.StartAsync()
}
