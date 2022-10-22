.PHONY: r rr b br fmt lint

r: b
	cargo +nightly run --package wpi32

rr: br
	cargo +nightly run --release --package wpi32

b: fmt
	cargo +nightly build

br: fmt
	cargo +nightly build --release --package wpi32

fmt:
	cargo +nightly fmt

lint:
	cargo +nightly clippy
