package static

import "embed"

//go:embed css favicon.png robots.txt img sitemap.xml
var FS embed.FS
