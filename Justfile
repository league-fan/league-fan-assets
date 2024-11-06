set dotenv-load

run:
    cargo run

debug:
    RUST_LOG=INFO cargo run > target/debug.log

test:
    cargo test

test-r2:
    cargo test r2client::tests

lint:
    cargo fmt
    cargo fix --allow-dirty