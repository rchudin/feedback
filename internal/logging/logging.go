package logging

import "net/http"

// Logger ...
type Logger interface {
	Info(string) error
	Error(string) error
	InfoHandler(http.ResponseWriter, *http.Request)
	PanicHandler(http.ResponseWriter, *http.Request, interface{})
}
