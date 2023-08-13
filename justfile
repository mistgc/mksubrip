_default:
  @just --list

run mode="debug":
    {{ if mode == "debug" { "cargo run" } else { "cargo run --" + mode } }}

build mode="debug":
    {{ if mode == "debug" { "cargo build" } else { "cargo build --" + mode} }}
