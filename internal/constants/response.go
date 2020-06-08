package constants

import (
	"net/http"

	json "github.com/json-iterator/go"
)

// MessageError ...
type MessageError struct {
	ErrorString string `json:"error"`
}

// StructError ...
type StructError map[string]string

func (m *MessageError) Error() string { return m.ErrorString }

// Error ...
func (s *StructError) Error() string {
	b, err := json.Marshal(s)
	if err != nil {
		return ""
	}
	return string(b)
}

var (
	//ErrFileIsToLarge ...
	ErrFileIsToLarge = &MessageError{"file is to large"}

	//ErrValidation ...
	ErrValidation = &MessageError{"validation error"}

	//ErrMustBeAValidEmailAddress ...
	ErrMustBeAValidEmailAddress = &MessageError{"must be a valid email address"}

	// ErrNotMultipart ...
	ErrNotMultipart = &MessageError{ErrorString: http.ErrNotMultipart.Error()}
)
