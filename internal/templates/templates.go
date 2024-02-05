package templates

import (
	"bytes"
	"github.com/reaper47/heavy-metal-notifier/views"
	"html/template"
	"io/fs"
	"strings"
)

var (
	emailTemplates map[string]*template.Template
)

func init() {
	initEmailTemplates()
}

func initEmailTemplates() {
	emailTemplates = make(map[string]*template.Template)

	emailDir, err := fs.ReadDir(views.FS, "emails/transpiled")
	if err != nil {
		panic(err)
	}

	for _, entry := range emailDir {
		n := entry.Name()

		data, err := fs.ReadFile(views.FS, "emails/transpiled/"+n)
		if err != nil {
			panic(err)
		}
		data = bytes.ReplaceAll(data, []byte("[["), []byte("{{"))
		data = bytes.ReplaceAll(data, []byte("]]"), []byte("}}"))

		tmpl := template.Must(template.New(n).Funcs(template.FuncMap{
			"nl2br": nl2br,
		}).Parse(string(data)))
		if tmpl == nil || tmpl.Tree == nil || tmpl.Tree.Root == nil {
			panic("template or tree or root of " + entry.Name() + " is nil")
		}
		emailTemplates[n] = tmpl
	}
}

func nl2br(text string) template.HTML {
	return template.HTML(strings.ReplaceAll(template.HTMLEscapeString(text), "\n", "<br />"))
}
