package bot

// Bot ...
type Bot interface {
	SendMessage(string, string) (int, error)
	SenFile([]byte, string, *int, *string) error
}
