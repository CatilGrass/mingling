#!/bin/bash

# Change to the directory where the script is located
cd "$(dirname "$0")"

# Run cargo clippy in mingling_core
cd mingling_core
cargo clippy --fix --allow-dirty
cd ..

# Run cargo clippy in mingling_macros
cd mingling_macros
cargo clippy --fix --allow-dirty
cd ..

# Run cargo clippy in mingling
cd mingling
cargo clippy --fix --allow-dirty
cd ..
