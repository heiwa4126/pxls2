.PHONY: run test check clean build release

run:
	cargo run

test:
	cargo test

check:
	cargo check

clean:
	cargo clean
	find . -type f -name \*~ -print0 | xargs -0 -r rm
	rm -f tmp/* Book1.xlsx updates_db.yaml
	rm -rf Cargo.lock target

build:
	cargo build

release:
	PKG_REVISION="$$(git rev-parse --short HEAD)" \
	RUSTFLAGS="-C link-arg=-s" \
	cargo build --release
