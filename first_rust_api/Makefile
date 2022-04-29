.PHONY: run build lint test deploy fmt

APP_NAME = first_rust_api
CARGO_EDITION ?= 2021

RELEASE_TARGET = target/release/${APP_NAME}

UNIT_TESTS = src/version/*.rs

run: build
	cargo run

fmt: src/*.rs tests/*.rs ${UNIT_TESTS}
	rustfmt --edition ${CARGO_EDITION} src/main.rs

build: fmt ${RELEASE_TARGET}

lint:
	cargo clippy

test:
	cargo test

deploy:
	if [[ -f $${artifactory_registry_cargo_config_toml} ]]; then cp $${artifactory_registry_cargo_config_toml} ~/.cargo/config; fi
	if [[ -f $${artifactory_registry_cargo_credentials_toml} ]]; then cp $${artifactory_registry_cargo_credentials_toml} ~/.cargo/credentials; fi
	cargo publish

${RELEASE_TARGET}: src/*.rs tests/*.rs Cargo.toml Cargo.lock
	cargo build --release
