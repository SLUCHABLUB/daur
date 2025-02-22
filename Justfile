export CARGO_TERM_COLOR := "always"
export RUST_BACKTRACE := "1"

default:
    @just --list

run:
    @just run-tui

run-tui:
    cargo run -p daur-tui

check:
    cargo +nightly fmt
    cargo clippy
    cargo test
