package handlers

import (
	"feedback/internal/constants"
	"net/http"

	json "github.com/json-iterator/go"
)

// Respond ...
func Respond(w http.ResponseWriter, r *http.Request, code int, data interface{}) {
	w.Header().Set(constants.ContentType, constants.ApplicationJson)
	w.WriteHeader(code)
	if data != nil {
		_ = json.NewEncoder(w).Encode(data)
	}
	return
}

// Error ...
func Error(w http.ResponseWriter, r *http.Request, code int, msg error) {
	if msg != nil {
		Respond(w, r, code, msg)
		return
	} else {
		http.Error(w, http.StatusText(code), code)
		return
	}
}

// Warning ...
func Warning(w http.ResponseWriter, r *http.Request, code int, err []string) {
	Respond(w, r, code, err)
}

// NotFound ...
func NotFound(w http.ResponseWriter, r *http.Request) {
	Error(w, r, http.StatusNotFound, nil)
	return
}
