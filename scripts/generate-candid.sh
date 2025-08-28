#!/bin/bash

PACKAGE_NAME=$1
CANISTER_NAME=${2:-$PACKAGE_NAME}

check_command_exists() {
    COMMAND=$1
    URL=$2

    if ! command -v $COMMAND >/dev/null 2>&1
    then
        echo "'$COMMAND' is required but could not be found, please install it and try again ($2)"
        exit 1
    fi
}

check_command_exists jq https://jqlang.org/download
check_command_exists candid-extractor https://crates.io/crates/candid-extractor

cargo build --target wasm32-unknown-unknown --release --package $PACKAGE_NAME

TARGET_DIR=$(cargo metadata --format-version 1 | jq -r ".target_directory")

WASM_PATH=$TARGET_DIR/wasm32-unknown-unknown/release/$CANISTER_NAME.wasm

candid-extractor $WASM_PATH