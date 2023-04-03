package templates

import (
	"context"
	"fmt"
	"github.com/Boostport/mjml-go"
	"github.com/oxtoacart/bpool"
	"github.com/reaper47/heavy-metal-notifier/views"
	"html/template"
	"io/fs"
	"log"
	"net/http"
	"path/filepath"
	"strings"
)

var (
	templates      map[string]*template.Template
	emailTemplates map[string]*template.Template
	bufPool        *bpool.BufferPool
)

func init() {
	templates = make(map[string]*template.Template)
	bufPool = bpool.NewBufferPool(64)

	dirs, err := fs.ReadDir(views.FS, ".")
	if err != nil {
		panic(err)
	}

	for _, entry := range dirs {
		if entry.IsDir() {
			continue
		}

		n := entry.Name()
		templates[n] = template.Must(template.New("main").ParseFS(views.FS, n, "layouts/*.gohtml"))
	}

	initEmailTemplates()
}

func initEmailTemplates() {
	emailTemplates = make(map[string]*template.Template)

	emailDir, err := fs.ReadDir(views.FS, "emails")
	if err != nil {
		panic(err)
	}

	for _, entry := range emailDir {
		n := entry.Name()

		if filepath.Ext(n) == ".mjml" {
			tmpl := template.Must(template.New(n).ParseFS(views.FS, "emails/"+n))

			html, err := mjml.ToHTML(context.Background(), tmpl.Tree.Root.String(), mjml.WithMinify(true))
			if err != nil {
				log.Fatal(err)
			}
			html = strings.ReplaceAll(html, "[[", "{{")
			html = strings.ReplaceAll(html, "]]", "}}")

			emailTemplates[n] = template.Must(template.New(n).Funcs(template.FuncMap{
				"nl2br": nl2br,
			}).Parse(html))
		}
	}
}

func nl2br(text string) template.HTML {
	return template.HTML(strings.ReplaceAll(template.HTMLEscapeString(text), "\n", "<br />"))
}

// Render is a wrapper for template.ExecuteTemplate.
func Render(w http.ResponseWriter, name string, data any) error {
	tmpl, ok := templates[name]
	if !ok {
		err := fmt.Errorf("the template %s does not exist", name)
		http.Error(w, fmt.Sprintf("the template %s does not exist", name), http.StatusInternalServerError)
		return err
	}

	buf := bufPool.Get()
	defer bufPool.Put(buf)

	if err := tmpl.Execute(buf, data); err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return err
	}

	w.Header().Set("Content-Type", "text/html; charset=utf-8")
	buf.WriteTo(w)
	return nil
}

// RenderEmail is a wrapper for template.ExecuteTemplate on email templates.
func RenderEmail(name string, data any) (string, error) {
	tmpl, ok := emailTemplates[name]
	if !ok {
		return "", fmt.Errorf("the template %s does not exist", name)
	}

	buf := bufPool.Get()
	defer bufPool.Put(buf)

	if err := tmpl.Execute(buf, data); err != nil {
		return "", err
	}
	return buf.String(), nil
}
