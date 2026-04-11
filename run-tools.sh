#!/bin/bash

cd "$(dirname "$0")" || exit 1

if [ $# -eq 0 ]; then
    echo "Available:"
    if [ -d "dev_tools/src/bin" ]; then
        for file in dev_tools/src/bin/*.rs; do
            if [ -f "$file" ]; then
                basename "$file" .rs
            fi
        done
    else
        echo "Warning: dev_tools/src/bin directory does not exist"
    fi
    exit 1
fi

target_bin="$1"
target_file="dev_tools/src/bin/${target_bin}.rs"

if [ ! -f "$target_file" ]; then
    echo "Error: target file '$target_file' does not exist"
    exit 1
fi

cargo run --manifest-path dev_tools/Cargo.toml --bin "$1"
