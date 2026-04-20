#!/bin/bash

find . -name "Cargo.toml" -type f | while read -r cargo_file; do
    project_dir=$(dirname "$cargo_file")
    echo "Run \`cargo clippy\` in \`$project_dir\` ..."
    (cd "$project_dir" && cargo clippy)
done
