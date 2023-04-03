.ONESHELL:

build:
	go generate ./...
	go build -ldflags="-s -w" -o bin/metal main.go

run:
	go run main.go serve

test:
	go test ./...

%:
	@:

.PHONY: release build run test