.ONESHELL:

build:
	go generate ./...
	go build -ldflags="-s -w" -o bin/metal main.go

release:
ifdef tag
		sh build.sh github.com/reaper47/heavy-metal-notifier $(tag)
		gh release create $(tag) ./release/$(tag)/*
else
		@echo 'Add the tag argument, i.e. `make release tag=v1.0.0`'
endif

run:
	go run main.go serve

test:
	go test ./...

%:
	@:

.PHONY: release build run test