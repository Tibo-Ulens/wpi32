.PHONY: b br t fmt lint doc docopen doctest

b: fmt
	cargo +nightly build --workspace

br: fmt
	cargo +nightly build --release --workspace

t: fmt
	cargo +nightly test --workspace

fmt:
	cargo +nightly fmt

lint:
	cargo +nightly clippy

doc:
	cargo +nightly doc --no-deps --document-private-items --workspace --all-features

docopen:
	cargo +nightly doc --no-deps --document-private-items --workspace --all-features --open

doctest:
	cargo +nightly test --doc --workspace
