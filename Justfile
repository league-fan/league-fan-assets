set dotenv-load

run:
    cargo run

debug:
    RUST_LOG=DEBUG cargo run > target/debug.log

test:
    cargo test

test-r2:
    cargo test r2client::tests::test_upload_file -- --exact

lint:
    cargo fmt
    cargo fix --allow-dirty