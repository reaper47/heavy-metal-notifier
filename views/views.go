package views

import "embed"

//go:embed *.gohtml layouts/*.gohtml emails/transpiled/*.gohtml
var FS embed.FS
