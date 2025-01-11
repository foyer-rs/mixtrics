SHELL := /bin/bash
.PHONY: ffmt check test machete all

all: ffmt check test machete

check:
	typos
	cargo sort -w
	cargo fmt --all
	cargo clippy --all-targets --all-features

test:
	RUST_BACKTRACE=1 cargo nextest run --all --all-features
	RUST_BACKTRACE=1 cargo test --doc --all-features
	RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --all-features --no-deps

machete:
	cargo machete

ffmt:
	cargo +nightly fmt --all -- --config-path rustfmt.nightly.toml