.PHONY: run build lint test deploy fmt

APP_NAME = first_rust_api
CARGO_EDITION ?= 2021

RELEASE_TARGET = target/release/${APP_NAME}

UNIT_TESTS = src/version/*.rs

run: build
	cargo run

fmt: src/*.rs tests/*.rs ${UNIT_TESTS}
	rustfmt --edition ${CARGO_EDITION} src/main.rs

debug: fmt ${DEBUG_TARGET}

build: fmt ${RELEASE_TARGET}

lint:
	cargo clippy

test:
	cargo test

deploy:
	scripts/publish.sh

${DEBUG_TARGET}: src/*.rs tests/*.rs Cargo.toml Cargo.lock
	cargo build --release

${RELEASE_TARGET}: src/*.rs tests/*.rs Cargo.toml Cargo.lock
	cargo build --release
