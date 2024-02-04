package views

import "embed"

//go:embed *.gohtml emails/transpiled/*.gohtml
var FS embed.FS
