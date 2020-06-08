BINARY := feedback

.PHONY: build
build:
	go build -o build/ -v ./cmd/$(BINARY)


.PHONY: test
test:
	go test -v -timeout 30s ./...


.DEFAULT_GOAL := build
