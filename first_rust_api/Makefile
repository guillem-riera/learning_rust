.PHONY: run build lint test deploy

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

deploy:
	if [[ -f $${artifactory_registry_cargo_config_toml} ]]; then cp $${artifactory_registry_cargo_config_toml} ~/.cargo/config.toml; fi
	if [[ -f $${artifactory_registry_cargo_credentials_toml} ]]; then cp $${artifactory_registry_cargo_credentials_toml} ~/.cargo/credentials.toml; fi
	cargo publish

${RELEASE_TARGET}: src/*.rs tests/*.rs Cargo.toml Cargo.lock
	cargo build --release
