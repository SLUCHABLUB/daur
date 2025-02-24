export CARGO_TERM_COLOR := "always"
export RUST_BACKTRACE := "1"
log_file := env("CARGO_TARGET_DIR", "target") / "log.txt"

default:
    @just --list --justfile {{justfile()}}

run:
    @just run-tui

run-tui:
    @-rm {{log_file}}
    @cargo run -p daur-tui 2> {{log_file}} || cat {{log_file}}

check:
    cargo +nightly fmt
    cargo clippy
    cargo test
