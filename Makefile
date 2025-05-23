#!/usr/bin/make -f

.PHONY: test
test:
	cargo nextest run -r --target wasm32-wasip1

.PHONY: build
build:
	fastly compute build -y
