package feedback

import (
	"feedback/internal/bot/telegrambot"
	"feedback/internal/config"
	"feedback/internal/logging/zap"
	"feedback/internal/server"
	"feedback/internal/validate/validator"
	"net/http"
	"strconv"
)

// Start ...
func Start(conf *config.Config) error {
	logger, err := zap.NewLogger(conf.Logging.Path)
	if err != nil {
		return err
	}

	bot, err := telegrambot.NewBot(conf.TelegramBot.Token, conf.TelegramBot.ChatID)
	if err != nil {
		return err
	}

	srv := server.NewServer(logger, bot, validator.NewValidator())

	// Start server
	return http.ListenAndServe(":"+strconv.Itoa(conf.Port), srv)
}
