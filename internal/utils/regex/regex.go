package regex

import "regexp"

// Email is the regex used to verify whether an email address is valid.
var Email = regexp.MustCompile(
	"^[\\w.!#$%&'*+/=?^_`{|}~-]+@\\w(?:[\\w-]{0,61}\\w)?(?:\\.\\w(?:[\\w-]{0,61}\\w)?)*$",
)
