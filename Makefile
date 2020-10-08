run:
	cargo run

test:
	cargo test

check:
	cargo check

build:
	cargo build

release:
	PKG_REVISION="$$(git rev-parse --short HEAD)" \
	RUSTFLAGS="-C link-arg=-s" \
	cargo check --release