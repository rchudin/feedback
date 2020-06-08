package server

import (
	"bytes"
	"feedback/internal/constants"
	"feedback/internal/handlers"
	"io"
	"net/http"

	"github.com/julienschmidt/httprouter"
)

func (s *Server) handlerFeedbackStatus() httprouter.Handle {
	return func(w http.ResponseWriter, r *http.Request, _ httprouter.Params) {
		handlers.Respond(w, r, http.StatusOK, "ok")
	}
}

// HandlerFeedback ...
func (s *Server) handlerFeedback() httprouter.Handle {
	type request struct {
		Subject  string `json:"subject" validate:"required,max=500"`
		Username string `json:"username" validate:"required,max=256"`
		Email    string `json:"email" validate:"omitempty,email"`
		Phone    string `json:"phone" validate:"max=256"`
		Message  string `json:"message" validate:"omitempty,max=3000"`
	}

	return func(w http.ResponseWriter, r *http.Request, _ httprouter.Params) {
		// Check message size (Проверка размера сообщения)
		r.Body = http.MaxBytesReader(w, r.Body, 40<<20+1024)
		if err := r.ParseMultipartForm(40 << 20); err != nil {
			if err == http.ErrNotMultipart {
				handlers.Error(w, r, http.StatusBadRequest, constants.ErrNotMultipart)
				return
			}
			handlers.Error(w, r, http.StatusBadRequest, constants.ErrFileIsToLarge)
			return
		}
		request := &request{}

		request.Subject = r.FormValue("subject")
		request.Username = r.FormValue("username")
		request.Email = r.FormValue("email")
		request.Phone = r.FormValue("phone")
		request.Message = r.FormValue("message")

		if ok, err := s.validate.Struct(request); err != nil {
			if !ok {
				handlers.Error(w, r, http.StatusBadRequest, constants.ErrValidation)
				return
			}

			handlers.Respond(w, r, http.StatusBadRequest, err)
			return
		}

		txt := "*Subject:* _" + request.Subject + "_\n"
		txt += "*Username:* _" + request.Username + "_\n"
		if len(request.Email) > 0 {
			txt += "*Email:* _" + request.Email + "_\n"
		}
		if len(request.Phone) > 0 {
			txt += "*Phone:* _" + request.Phone + "_\n"
		}
		if len(request.Message) > 0 {
			txt += "*Message* _" + request.Message + "_"
		}

		idMsg, err := s.bot.SendMessage(txt, "markdown")

		if err != nil {
			handlers.Error(w, r, http.StatusBadRequest, constants.ErrValidation)
			return
		}

		file, handler, err := r.FormFile("file")

		if err != nil {
			handlers.Warning(w, r, http.StatusOK, []string{"file not sent"})
			return
		}
		defer file.Close()

		buf := bytes.NewBuffer(nil)
		if _, err := io.Copy(buf, file); err == nil {
			if err = s.bot.SenFile(buf.Bytes(), handler.Filename, &idMsg, nil); err != nil {
				handlers.Warning(w, r, http.StatusOK, []string{"file not sent"})
				return
			}

		} else {
			handlers.Warning(w, r, http.StatusOK, []string{"file not sent"})
			return
		}

		handlers.Respond(w, r, http.StatusOK, "ok")
	}
}
