#!/usr/bin/env bash

function get_current_version() {
  export CURRENT_VERSION=$(dasel select -r toml '.package.version' -f Cargo.toml)
}

function get_version_parts() {
  export MAJOR_VERSION="${CURRENT_VERSION%%.*}"
  export PATCH_VERSION=${CURRENT_VERSION##*.}
  export MAJOR_MINOR=${CURRENT_VERSION%.*}
  export MINOR_VERSION=${MAJOR_MINOR#*.}
}

function next_version() {
  export NEXT_VERSION="${MAJOR_VERSION}.${MINOR_VERSION}.${PATCH_VERSION}"
}

function bump_patch_version() {
    NEXT_PATCH=$(( PATCH_VERSION + 1 ))
    export PATCH_VERSION=${NEXT_PATCH}
}

function bump_minor_version() {
    NEXT_MINOR=$(( MINOR_VERSION + 1 ))
    export PATCH_VERSION=0
    export MINOR_VERSION=${NEXT_MINOR}
}

function bump_major_version() {
    NEXT_MAJOR=$(( MAJOR_VERSION + 1 ))
    export PATCH_VERSION=0
    export MINOR_VERSION=0
    export MAJOR_VERSION=${NEXT_MAJOR}
}

function check_for_merge() {
  export COMMIT_MESSAGE=$(git --no-pager log --format=%B -1 | head -n 1 || tr -d '[:space:]')
  case "${COMMIT_MESSAGE}" in
    *Merge*)
      echo "[Info] Found a merge commit"
      export IS_A_MERGE="True"
      ;;
    *)
      export IS_A_MERGE="False"
      ;;
  esac
}

function bump_version_according_to_message() {
    check_for_merge
    if [[ "${IS_A_MERGE}" == "True" ]]; then
      case "${COMMIT_MESSAGE}" in
        *feature/*)
          bump_minor_version
          ;;
        *fix*/)
          bump_patch_version
          ;;
        *major*|*Major*|*Breaking*|*breaking*)
          bump_major_version
          ;;
      esac
    fi
}

function main() {
    get_current_version
    get_version_parts
    # Decide how to bump
    bump_version_according_to_message
    next_version
}

main

# Version change test
