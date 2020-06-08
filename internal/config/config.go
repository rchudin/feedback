package config

import (
	"errors"

	"github.com/alexflint/go-arg"
)

type logging struct {
	Path string `toml:"path"`
}

type telegramBot struct {
	Token  string `toml:"token"`
	ChatID int64  `toml:"chat_id"`
}

// Config ...
type Config struct {
	Port        int         `toml:"port"`
	TelegramBot telegramBot `toml:"telegram_bot"`
	Logging     logging     `toml:"logging"`
}

// NewDefaultConfig ...
func NewDefaultConfig() *Config {
	return &Config{
		Port: 3911,
		TelegramBot: telegramBot{
			Token:  "",
			ChatID: 0,
		},
		Logging: logging{
			Path: "log.json",
		},
	}
}

// LoadConfigs ...
func LoadConfigs() (*Config, error) {
	var args struct {
		Input          string   `arg:"positional"`
		Output         []string `arg:"positional"`
		Log            *string  `arg:"-l" help:"log file path"`
		Port           *int     `arg:"-p" help:"server port"`
		TelegramToken  string   `help:"telegram bot token"`
		TelegramChatID int64    `help:"telegram chat id"`
	}

	arg.MustParse(&args)

	conf := NewDefaultConfig()

	if len(args.TelegramToken) <= 0 {
		return nil, errors.New("missing telegram token")
	} else {
		conf.TelegramBot.Token = args.TelegramToken
	}

	if args.TelegramChatID == conf.TelegramBot.ChatID {
		return nil, errors.New("missing telegram chat id")
	} else {
		conf.TelegramBot.ChatID = args.TelegramChatID
	}

	if args.Log != nil {
		conf.Logging.Path = *args.Log
	}

	if args.Port != nil {
		conf.Port = *args.Port
	}

	return conf, nil
}
