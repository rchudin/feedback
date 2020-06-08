package telegrambot

import (
	"feedback/internal/bot"

	tgbotapi "github.com/go-telegram-bot-api/telegram-bot-api"
)

// TgBot ...
type TgBot struct {
	api    *tgbotapi.BotAPI
	chatID int64
}

var _ bot.Bot = (*TgBot)(nil)

// NewBot ...
func NewBot(token string, chatID int64) (*TgBot, error) {
	api, err := tgbotapi.NewBotAPI(token)

	if err != nil {
		return nil, err
	}

	return &TgBot{
		api:    api,
		chatID: chatID,
	}, nil
}

// SendMessage ...
func (bot *TgBot) SendMessage(text string, parseMode string) (int, error) {

	msg := tgbotapi.NewMessage(bot.chatID, text)
	msg.ParseMode = parseMode
	articles, err := bot.api.Send(msg)

	if err != nil {
		return 0, err
	}

	return articles.MessageID, err
}

// SenFile ...
func (bot *TgBot) SenFile(b []byte, name string, replyID *int, caption *string) error {

	file := tgbotapi.FileBytes{
		Name:  name,
		Bytes: b,
	}

	msg := tgbotapi.NewDocumentUpload(bot.chatID, file)

	if replyID != nil {
		msg.ReplyToMessageID = *replyID
	}

	if caption != nil {
		msg.Caption = *caption
	}

	_, err := bot.api.Send(msg)

	if err != nil {
		return err
	}

	return nil
}
