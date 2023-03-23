alias r := run
alias b := build
alias t := test
alias l := lint
alias c := check

run subcommand:
    cd {{invocation_directory()}} && cargo run -- -vv {{subcommand}}

build:
    cargo build

test:
    cargo test --all

lint:
    cargo clippy -- -D warnings

check: build test lint

