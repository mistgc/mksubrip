_default:
    @just --list

# Build and run the project
run mode="debug":
    {{ if mode == "debug" { "RUST_LOG=debug cargo run" } else { "RUST_LOG=debug cargo run --" + mode } }}

# Build the project
build mode="debug":
    {{ if mode == "debug" { "RUST_LOG=debug cargo build" } else { "cargo build --" + mode } }}

# Run all test cases
test:
    cargo test

# Generate document
doc:
    cargo doc

# Turn the "cargo watch" mode on
watch command="check":
    #!/bin/env bash

    deps=(
        "cargo watch"
    )

    check_deps() {
        echo "Start checking dependences..."
        for dep in "${deps[@]}"; do
            if [[ $? == "127" ]]; then
                echo "${dep} is not be found!"
                exit 1
            else
                echo "${dep} is found."
            fi
        done
        echo "Finish checking dependences..."
    }

    check_deps

    cargo watch -c -x "{{ command }}"

# Lint rust sources
clippy:
    cargo clippy --all-targets --all-features --tests --benches -- -D warnings

fmt:
    cargo fmt -- --check

check:
    cargo check
