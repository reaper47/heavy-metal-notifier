package server

import (
	"encoding/base64"
	"fmt"
	"github.com/go-chi/chi/v5"
	"metal-releases/internal/app"
	"metal-releases/internal/constants"
	"metal-releases/internal/email"
	"metal-releases/internal/services"
	"metal-releases/internal/templates"
	"metal-releases/internal/utils/regex"
	"metal-releases/static"
	"net/http"
	"strings"
)

// New creates a new, non-configured server.
func New(service services.Service) *Server {
	srv := &Server{
		Service: service,
	}
	srv.mountHandlers()
	return srv
}

// Server is the web application's configuration object.
type Server struct {
	Router  *chi.Mux
	Service services.Service
}

func (s *Server) mountHandlers() {
	r := chi.NewRouter()

	r.Get("/", index)
	r.Get("/about", about)
	r.Get("/contact", contact)
	r.Post("/contact", postContact)
	r.Get("/privacy", privacy)
	r.Get("/start", start)
	r.Post("/start", s.postStart)
	r.Get("/stop", s.stop)
	r.Get("/tos", tos)

	r.Get("/sitemap", sitemap)
	r.Get("/favicon.ico", favicon)
	r.Get("/robots.txt", robots)
	r.Mount("/static", http.StripPrefix("/static", http.FileServer(http.FS(static.FS))))

	s.Router = r
}

func index(w http.ResponseWriter, _ *http.Request) {
	_ = templates.Render(w, "index.gohtml", templates.Data{
		ShowNav:    true,
		IsHomePage: true,
	})
}

func about(w http.ResponseWriter, _ *http.Request) {
	_ = templates.Render(w, "about.gohtml", templates.Data{
		ShowNav:     true,
		IsAboutPage: true,
	})
}

func contact(w http.ResponseWriter, _ *http.Request) {
	_ = templates.Render(w, "contact.gohtml", templates.Data{
		ShowNav:       true,
		IsContactPage: true,
	})
}

func postContact(w http.ResponseWriter, r *http.Request) {
	to := r.FormValue("email")
	message := r.FormValue("message")

	if !regex.Email.Match([]byte(to)) || message == "" {
		http.Error(w, "The contact form must not be empty.", http.StatusBadRequest)
		return
	}

	email.Send(to, constants.EmailContact, templates.EmailData{
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

func privacy(w http.ResponseWriter, _ *http.Request) {
	_ = templates.Render(w, "privacy.gohtml", templates.Data{
		ShowNav: true,
	})
}

func start(w http.ResponseWriter, _ *http.Request) {
	_ = templates.Render(w, "start.gohtml", nil)
}

func (s *Server) postStart(w http.ResponseWriter, r *http.Request) {
	userEmail := r.FormValue("email")
	if userEmail == "" {
		w.WriteHeader(http.StatusBadRequest)
		w.Write([]byte("your email is required"))
		return
	}

	if err := s.Service.Register(userEmail); err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		_ = templates.Render(w, "simple-screen.gohtml", templates.NoDuplicateUsersError)
		return
	}

	email.Send(userEmail, constants.EmailIntro, &templates.EmailData{
		EmailBase64: base64.StdEncoding.EncodeToString([]byte(userEmail)),
		Name:        strings.Split(userEmail, "@")[0],
		URL:         app.Config.URL,
	})

	_ = templates.Render(w, "start-success.gohtml", nil)
}

func (s *Server) stop(w http.ResponseWriter, r *http.Request) {
	idBase64 := r.URL.Query().Get("id")
	if idBase64 == "" {
		http.Redirect(w, r, "/", http.StatusSeeOther)
		return
	}

	id, err := base64.StdEncoding.DecodeString(idBase64)
	if err != nil {
		email.Send(app.Config.Email.From, constants.EmailErrorAdmin, templates.DataError{
			Text: fmt.Sprintf("error decoding base64 email id: %q", err),
		})
		_ = templates.Render(w, "simple-screen.gohtml", templates.StopError)
		return
	}

	userEmail := string(id)
	if err := s.Service.Unregister(userEmail); err != nil {
		email.Send(app.Config.Email.From, constants.EmailErrorAdmin, templates.DataError{
			Text: fmt.Sprintf("error deleting all of %q data related to: %q", userEmail, err),
		})
		_ = templates.Render(w, "simple-screen.gohtml", templates.StopError)
		return
	}

	email.Send(userEmail, constants.EmailEndOfService, templates.EmailData{Name: strings.Split(userEmail, "@")[0]})

	_ = templates.Render(w, "stop-success.gohtml", nil)
}

func tos(w http.ResponseWriter, r *http.Request) {
	_ = templates.Render(w, "tos.gohtml", templates.Data{
		ShowNav: true,
	})
}

func sitemap(w http.ResponseWriter, _ *http.Request) {
	serveFile(w, "sitemap.xml", "application/xml")
}

func favicon(w http.ResponseWriter, _ *http.Request) {
	serveFile(w, "favicon.png", "image/x-icon")
}

func robots(w http.ResponseWriter, _ *http.Request) {
	serveFile(w, "robots.txt", "text/plain")
}

func serveFile(w http.ResponseWriter, fileName, contentType string) {
	f, err := static.FS.ReadFile(fileName)
	if err != nil {
		w.WriteHeader(http.StatusNotFound)
		return
	}

	w.Header().Set("Content-Type", contentType)
	w.Write(f)
}
