package telegrambot_test

import (
	"feedback/internal/bot/telegrambot"
	"io/ioutil"
	"os"
	"path/filepath"
	"testing"

	"github.com/stretchr/testify/assert"
)

// TestBot ...

func TestBot_SendMeMessage(t *testing.T) {
	bot := telegrambot.TestBot(t, "test_config.toml")

	_, err := bot.SendMessage("test text", "markdown")

	assert.NoError(t, err)
}

func TestBot_SendMeFile(t *testing.T) {
	bot := telegrambot.TestBot(t, "test_config.toml")

	dir := "test_config.toml"
	file, err := os.Open(dir)
	assert.NoError(t, err)

	b, err := ioutil.ReadAll(file)
	assert.NoError(t, err)

	err = bot.SenFile(b, filepath.Base(dir), nil, nil)
	assert.NoError(t, err)
}

func TestBot_SendMeMessageAndFile(t *testing.T) {
	bot := telegrambot.TestBot(t, "test_config.toml")
	subject := "test text"
	id, err := bot.SendMessage(subject, "markdown")
	assert.NoError(t, err)

	dir := "test_config.toml"
	file, err := os.Open(dir)
	assert.NoError(t, err)
	b, err := ioutil.ReadAll(file)
	assert.NoError(t, err)

	err = bot.SenFile(b, filepath.Base(dir), &id, &subject)
	assert.NoError(t, err)
}
