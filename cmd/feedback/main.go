package main

import (
	"feedback/internal/config"
	"feedback/internal/feedback"
	"log"
)

func main() {
	conf, err := config.LoadConfigs()
	if err != nil {
		log.Panicf("error: %v", err)
	}

	log.Printf("listening http://127.0.0.1:%v ...", conf.Port)
	if err := feedback.Start(conf); err != nil {
		log.Panicf("error: %v", err)
	}
}
