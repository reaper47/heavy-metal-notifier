package views

import "embed"

//go:embed *.gohtml layouts/*.gohtml emails/*.mjml
var FS embed.FS
