#!/usr/bin/make -f

.PHONY: test
test:
	CARGO_TARGET_WASM32_WASIP1_RUNNER="viceroy run -C fastly.toml --" cargo nextest run -r --target wasm32-wasip1