run:
    cargo run

debug:
    RUST_LOG=DEBUG cargo run > target/debug.log

test:
    cargo test

lint:
    cargo fmt
    cargo fix --allow-dirty