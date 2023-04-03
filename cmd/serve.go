package cmd

import (
	"context"
	"errors"
	"github.com/reaper47/heavy-metal-notifier/internal/app"
	"github.com/reaper47/heavy-metal-notifier/internal/jobs"
	"github.com/reaper47/heavy-metal-notifier/internal/server"
	"github.com/reaper47/heavy-metal-notifier/internal/services"
	"github.com/spf13/cobra"
	"log"
	"net/http"
	"os"
	"os/signal"
	"strconv"
	"syscall"
	"time"
)

var serveCmd = &cobra.Command{
	Use:   "serve",
	Short: "starts the web server",
	Long:  "Starts the web server.",
	Run: func(cmd *cobra.Command, args []string) {
		srv := server.New(services.NewSQLiteService())

		jobs.ScheduleFetchCalendar()
		jobs.ScheduleCheckReleases(srv.Service)

		addr := "0.0.0.0:" + strconv.Itoa(app.Config.Port)

		httpServer := &http.Server{
			Addr:              addr,
			Handler:           srv.Router,
			ReadTimeout:       15 * time.Second,
			ReadHeaderTimeout: 15 * time.Second,
			WriteTimeout:      15 * time.Second,
			IdleTimeout:       1 * time.Minute,
		}

		serverCtx, serverStopCtx := context.WithCancel(context.Background())

		sig := make(chan os.Signal, 1)
		signal.Notify(sig, syscall.SIGHUP, syscall.SIGINT, syscall.SIGTERM, syscall.SIGQUIT)
		go func() {
			<-sig

			shutdownCtx, shutdownCancel := context.WithTimeout(serverCtx, 30*time.Second)
			defer shutdownCancel()

			go func() {
				<-shutdownCtx.Done()
				if errors.Is(shutdownCtx.Err(), context.DeadlineExceeded) {
					log.Fatal("graceful shutdown timed out.. forcing exit.")
				}
			}()

			if err := httpServer.Shutdown(shutdownCtx); err != nil {
				log.Fatal(err)
			}
			serverStopCtx()
		}()

		log.Println("Serving on http://" + addr)
		if err := httpServer.ListenAndServe(); err != nil {
			log.Fatal(err)
		}

		<-serverCtx.Done()
	},
}

func init() {
	rootCmd.AddCommand(serveCmd)
}
