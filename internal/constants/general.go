package constants

// DBName is the name of the Aspen METTE™ VFM database.
const DBName = "metal.db"

// ConfigFile is the name of the web application's configuration file. By convention,
// this file is named "config.json" and sits next to the database.
const ConfigFile = "config.json"

// Platform represents a music streaming platform.
type Platform string

// Platform constants for YouTube and Bandcamp.
const (
	PlatformYouTube  Platform = "YouTube"
	PlatformBandcamp Platform = "Bandcamp"
)
