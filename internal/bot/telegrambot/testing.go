package telegrambot

import (
	"testing"

	"github.com/BurntSushi/toml"
)

// TestBot ...
func TestBot(t *testing.T, pathConfig string) *TgBot {
	t.Helper()
	type botConfig struct {
		Token  string `toml:"token"`
		ChatID int64  `toml:"chat_id"`
	}

	var config botConfig

	_, err := toml.DecodeFile(pathConfig, &config)

	if err != nil {
		t.Fatal(err)
	}

	bot, err := NewBot(config.Token, config.ChatID)

	if err != nil {
		t.Fatal(err)
	}
	return bot
}
