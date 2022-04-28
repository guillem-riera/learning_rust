.PHONY: run build lint

APP_NAME = first_rust_api

RELEASE_TARGET = target/release/${APP_NAME}

run: build
	cargo run

build: ${RELEASE_TARGET}

lint: ${RELEASE_TARGET}
	cargo clippy

${RELEASE_TARGET}:
	cargo build --release
