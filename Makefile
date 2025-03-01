lint:
	cargo clippy -- -Dwarnings
	cargo fmt --check --verbose

test:
	cargo test --verbose

build:
	cargo build --verbose

run:
	cargo run

