package main

import (
	"github.com/reaper47/heavy-metal-notifier/internal/server"
	"github.com/reaper47/heavy-metal-notifier/internal/services"
	"github.com/urfave/cli/v2"
	"log"
	"os"
)

func main() {
	app := &cli.App{
		Commands: []*cli.Command{
			{
				Name:    "serve",
				Aliases: []string{"s"},
				Usage:   "starts the web server",
				Action: func(ctx *cli.Context) error {
					srv := server.NewServer(services.NewSQLiteService(), services.NewEmailService())
					srv.Run()
					return nil
				},
			},
		},
		Usage: "the ultimate heavy metal album releases notifier",
	}

	if err := app.Run(os.Args); err != nil {
		log.Fatal(err)
	}
}
