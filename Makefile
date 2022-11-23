.PHONY: r rr b br fmt lint

b: fmt
	cargo +nightly build --workspace

br: fmt
	cargo +nightly build --release --workspace

fmt:
	cargo +nightly fmt

lint:
	cargo +nightly clippy

doc:
	cargo +nightly doc --no-deps --document-private-items --workspace --all-features

docopen:
	cargo +nightly doc --no-deps --document-private-items --workspace --all-features --open
