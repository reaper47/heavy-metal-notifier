package app

import (
	"encoding/json"
	"github.com/go-co-op/gocron"
	"github.com/reaper47/heavy-metal-notifier/internal/constants"
	"github.com/reaper47/heavy-metal-notifier/internal/models"
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

// ConfigEmail holds email configuration variables.
type ConfigEmail struct {
	From           string `json:"from"`
	SendGridAPIKey string `json:"sendGridAPIKey"`
}

func init() {
	exe, err := os.Executable()
	if err != nil {
		log.Fatal(err)
	}

	xb, err := os.ReadFile(filepath.Join(filepath.Dir(exe), constants.ConfigFile))
	if err != nil {
		log.Fatal(err)
	}

	if err := json.Unmarshal(xb, &Config); err != nil {
		log.Fatal(err)
	}

	Scheduler = gocron.NewScheduler(time.Local)
	Scheduler.StartAsync()
}
