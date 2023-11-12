package server

import (
	"context"
	"encoding/base64"
	"errors"
	"github.com/go-chi/chi/v5"
	"github.com/reaper47/heavy-metal-notifier/internal/app"
	"github.com/reaper47/heavy-metal-notifier/internal/jobs"
	"github.com/reaper47/heavy-metal-notifier/internal/services"
	"github.com/reaper47/heavy-metal-notifier/internal/templates"
	"github.com/reaper47/heavy-metal-notifier/internal/utils/regex"
	"github.com/reaper47/heavy-metal-notifier/static"
	"log"
	"net/http"
	"os"
	"os/signal"
	"strconv"
	"strings"
	"syscall"
	"time"
)

// NewServer creates a Server.
func NewServer(repository services.RepositoryService, email services.EmailService) *Server {
	srv := &Server{
		Repository: repository,
		Email:      email,
	}
	srv.mountHandlers()
	return srv
}

// Server is the web application's configuration object.
type Server struct {
	Router     *chi.Mux
	Repository services.RepositoryService
	Email      services.EmailService
}

func (s *Server) Run() {
	jobs.ScheduleFetchCalendar()
	jobs.ScheduleCheckReleases(s.Repository, s.Email)
	jobs.ScheduleCleanUsers(s.Repository, s.Email)

	addr := app.Config.URL
	if app.Config.Port > 0 {
		addr += ":" + strconv.Itoa(app.Config.Port)
	}

	httpServer := &http.Server{
		Addr:              strings.TrimPrefix(addr, "http://"),
		Handler:           s.Router,
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

	log.Println("Serving on " + addr)
	if err := httpServer.ListenAndServe(); err != nil {
		log.Fatal(err)
	}

	<-serverCtx.Done()
}

func (s *Server) mountHandlers() {
	r := chi.NewRouter()

	r.Get("/", indexHandler)
	r.Get("/about", aboutHandler)
	r.Get("/confirm", s.confirmHandler)
	r.Get("/contact", contactHandler)
	r.Post("/contact", s.postContactHandler)
	r.Get("/privacy", privacyHandler)
	r.Get("/start", startHandler)
	r.Post("/start", s.postStartHandler)
	r.Get("/stop", s.stopHandler)
	r.Get("/tos", tosHandler)

	r.Get("/sitemap", sitemapHandler)
	r.Get("/favicon.ico", faviconHandler)
	r.Get("/robots.txt", robotsHandler)
	r.Mount("/static", http.StripPrefix("/static", http.FileServer(http.FS(static.FS))))

	s.Router = r
}

func indexHandler(w http.ResponseWriter, _ *http.Request) {
	_ = templates.Render(w, "index.gohtml", templates.Data{
		ShowNav:    true,
		IsHomePage: true,
	})
}

func aboutHandler(w http.ResponseWriter, _ *http.Request) {
	_ = templates.Render(w, "about.gohtml", templates.Data{
		ShowNav:     true,
		IsAboutPage: true,
	})
}

func (s *Server) confirmHandler(w http.ResponseWriter, r *http.Request) {
	idBase64 := r.URL.Query().Get("id")
	if idBase64 == "" {
		http.Redirect(w, r, "/", http.StatusSeeOther)
		return
	}

	id, err := base64.StdEncoding.DecodeString(idBase64)
	if err != nil {
		sendErrorAdminEmail(s.Email.Send, "confirmHandler.DecodeString", err)
		_ = templates.Render(w, "simple-screen.gohtml", templates.ConfirmError)
		return
	}

	userEmail := string(id)
	if err := s.Repository.Confirm(userEmail); err != nil {
		sendErrorAdminEmail(s.Email.Send, "Repository.Confirm for "+userEmail, err)
		_ = templates.Render(w, "simple-screen.gohtml", templates.ConfirmError)
		return
	}

	_ = templates.Render(w, "simple-screen.gohtml", templates.ConfirmSuccess)
}

func contactHandler(w http.ResponseWriter, _ *http.Request) {
	_ = templates.Render(w, "contact.gohtml", templates.Data{
		ShowNav:       true,
		IsContactPage: true,
	})
}

func (s *Server) postContactHandler(w http.ResponseWriter, r *http.Request) {
	to := r.FormValue("email")
	message := r.FormValue("message")

	if !regex.Email.Match([]byte(to)) || message == "" {
		http.Error(w, "The contact form must not be empty.", http.StatusBadRequest)
		return
	}

	s.Email.Send(to, templates.EmailContact, templates.EmailData{
		From:    to,
		Message: message,
	})

	w.WriteHeader(http.StatusAccepted)
	_ = templates.Render(w, "contact.gohtml", templates.Data{
		ShowNav:       true,
		IsContactPage: true,
		IsMessageSent: true,
	})
}

func privacyHandler(w http.ResponseWriter, _ *http.Request) {
	_ = templates.Render(w, "privacy.gohtml", templates.Data{
		ShowNav: true,
	})
}

func startHandler(w http.ResponseWriter, _ *http.Request) {
	_ = templates.Render(w, "start.gohtml", nil)
}

func (s *Server) postStartHandler(w http.ResponseWriter, r *http.Request) {
	users, err := s.Repository.Users()
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		_, _ = w.Write([]byte("Error fetching data from the database."))
		return
	}

	if len(users) > app.Config.Email.MaxNumberUsers {
		w.WriteHeader(http.StatusBadRequest)
		_ = templates.Render(w, "simple-screen.gohtml", templates.UserLimitReachedError)
		return
	}

	userEmail := r.FormValue("email")
	if userEmail == "" {
		w.WriteHeader(http.StatusBadRequest)
		_, _ = w.Write([]byte("your email is required"))
		return
	}

	if err := s.Repository.Register(userEmail); err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		_ = templates.Render(w, "simple-screen.gohtml", templates.NoDuplicateUsersError)
		return
	}

	s.Email.Send(userEmail, templates.EmailIntro, &templates.EmailData{
		EmailBase64: base64.StdEncoding.EncodeToString([]byte(userEmail)),
		Name:        strings.Split(userEmail, "@")[0],
		URL:         app.Config.URL,
	})

	_ = templates.Render(w, "start-success.gohtml", nil)
}

func (s *Server) stopHandler(w http.ResponseWriter, r *http.Request) {
	idBase64 := r.URL.Query().Get("id")
	if idBase64 == "" {
		http.Redirect(w, r, "/", http.StatusSeeOther)
		return
	}

	id, err := base64.StdEncoding.DecodeString(idBase64)
	if err != nil {
		sendErrorAdminEmail(s.Email.Send, "stopHandler.DecodeString for "+idBase64, err)
		_ = templates.Render(w, "simple-screen.gohtml", templates.StopError)
		return
	}

	userEmail := string(id)
	if err := s.Repository.Unregister(userEmail); err != nil {
		sendErrorAdminEmail(s.Email.Send, "stopHandler.Repository.Unregister for "+userEmail, err)
		_ = templates.Render(w, "simple-screen.gohtml", templates.StopError)
		return
	}

	s.Email.Send(userEmail, templates.EmailEndOfService, templates.EmailData{Name: strings.Split(userEmail, "@")[0]})

	_ = templates.Render(w, "stop-success.gohtml", nil)
}

func tosHandler(w http.ResponseWriter, _ *http.Request) {
	_ = templates.Render(w, "tos.gohtml", templates.Data{
		ShowNav: true,
	})
}

func sitemapHandler(w http.ResponseWriter, _ *http.Request) {
	serveFile(w, "sitemap.xml", "application/xml")
}

func faviconHandler(w http.ResponseWriter, _ *http.Request) {
	serveFile(w, "favicon.png", "image/x-icon")
}

func robotsHandler(w http.ResponseWriter, _ *http.Request) {
	serveFile(w, "robots.txt", "text/plain")
}

func serveFile(w http.ResponseWriter, fileName, contentType string) {
	f, err := static.FS.ReadFile(fileName)
	if err != nil {
		w.WriteHeader(http.StatusNotFound)
		return
	}

	w.Header().Set("Content-Type", contentType)
	_, _ = w.Write(f)
}
