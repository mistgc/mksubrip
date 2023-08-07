_default:
  @just --list

run mode="debug":
    {{ if mode == "debug" { "cargo run" } else { "cargo run --" + mode } }}
