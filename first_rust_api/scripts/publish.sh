#!/usr/bin/env bash

export REGISTRY=${REGISTRY:-"custom"}
export PACKAGE_NAME="$(cat Cargo.toml| dasel -r toml '.package.name')"
export PACKAGE_VERSION="$(cat Cargo.toml| dasel -r toml '.package.version')"

echo "[Info] Package Name: ${PACKAGE_NAME}, Version: ${PACKAGE_VERSION}"

CARGO_SEARCH_RESULT=$(cargo search --registry=${REGISTRY} "${PACKAGE_NAME}")
CRATE_IS_DEPLOYED=$(echo $CARGO_SEARCH_RESULT | grep "${PACKAGE_NAME} = ${PACKAGE_VERSION}")

if [[ -z "${CRATE_IS_DEPLOYED}" ]]; then
    cargo publish --registry=$REGISTRY
fi
