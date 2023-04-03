package email

import (
	"errors"
	"net/smtp"
)

type auth struct {
	username string
	password string
}

// Start begins an authentication with a server.
func (a *auth) Start(_ *smtp.ServerInfo) (string, []byte, error) {
	return "LOGIN", []byte{}, nil
}

// Next continues the authentication. The server has just sent
// the fromServer data.
func (a *auth) Next(fromServer []byte, more bool) ([]byte, error) {
	if more {
		switch string(fromServer) {
		case "Username:":
			return []byte(a.username), nil
		case "Password:":
			return []byte(a.password), nil
		default:
			return nil, errors.New("unknown fromServer")
		}
	}
	return nil, nil
}
