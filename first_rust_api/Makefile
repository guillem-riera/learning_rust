.PHONY: run build lint test

APP_NAME = first_rust_api

RELEASE_TARGET = target/release/${APP_NAME}

UNIT_TESTS = src/version/*.rs

run: build
	cargo run

build: ${RELEASE_TARGET}

lint: ${RELEASE_TARGET}
	cargo clippy

test:
	cargo test

${RELEASE_TARGET}: src/*.rs tests/*.rs
	cargo build --release
