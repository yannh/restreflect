#!/usr/bin/make -f

.PHONY: test
test:
	cargo nextest run -r --target wasm32-wasip1

.PHONY: build
build:
	cargo build --target wasm32-wasip1
