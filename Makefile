SHELL := /bin/bash
.PHONY: ffmt check test machete all

all: ffmt check test machete

check:
	typos
	cargo sort -w
	cargo fmt --all
	cargo clippy --all-targets --features prometheus,prometheus-client,prometheus-client_0_22,prometheus-client_0_23,opentelemetry_0_26,opentelemetry_0_27

test:
	RUST_BACKTRACE=1 cargo nextest run --all --features "prometheus,prometheus-client,prometheus-client_0_22,prometheus-client_0_23,opentelemetry_0_26,opentelemetry_0_27"
	RUST_BACKTRACE=1 cargo test --doc --features "prometheus,prometheus-client,prometheus-client_0_22,prometheus-client_0_23,opentelemetry_0_26,opentelemetry_0_27"
	RUSTDOCFLAGS="--cfg docsrs -D warnings" cargo +nightly doc --features "prometheus,prometheus-client,prometheus-client_0_22,prometheus-client_0_23,opentelemetry_0_26,opentelemetry_0_27" --no-deps

machete:
	cargo machete

ffmt:
	cargo +nightly fmt --all -- --config-path rustfmt.nightly.toml