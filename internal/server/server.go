package server

import (
	"net/http"

	"feedback/internal/bot"
	"feedback/internal/handlers"
	"feedback/internal/logging"
	"feedback/internal/validate"

	"github.com/julienschmidt/httprouter"
)

// Server ...
type Server struct {
	router   *httprouter.Router
	logger   logging.Logger
	validate validate.Validator
	bot      bot.Bot
}

var _ http.Handler = (*Server)(nil)

// NewServer ...
func NewServer(logger logging.Logger, bot bot.Bot, validate validate.Validator) *Server {
	srv := &Server{
		logger:   logger,
		bot:      bot,
		validate: validate,
	}

	srv.settingRouter()

	return srv
}

func (s *Server) ServeHTTP(w http.ResponseWriter, r *http.Request) {
	s.logger.InfoHandler(w, r)

	s.router.ServeHTTP(w, r)
}

func (s *Server) settingRouter() {
	s.router = httprouter.New()

	baseURL := "/api/feedback"

	s.router.NotFound = http.HandlerFunc(handlers.NotFound)
	s.router.PanicHandler = s.logger.PanicHandler

	s.router.GET(baseURL, s.handlerFeedbackStatus())
	s.router.POST(baseURL, s.handlerFeedback())
}
