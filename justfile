mod docker 'docker.just'
mod hurl 'tests/hurl.just'

run:
    cargo run

kill:
    #!/usr/bin/env bash
    lsof -ti:8111 | xargs -r kill -9 || true

alias v := verify

up:
    just docker stop
    just docker run
    just hurl test

verify:
    cargo fmt -- --check
    cargo check
    cargo clippy
    cargo test
    just docker run
    just hurl test
    just docker stop

fmt:
    cargo fmt
    cargo clippy --fix --allow-dirty
