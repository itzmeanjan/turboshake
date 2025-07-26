# Copied from https://github.com/itzmeanjan/rlnc/blob/9fd105c8a3b24aeea1d7f71746887b08ef447814/Makefile

.DEFAULT_GOAL := help

.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

BACKTRACE=RUST_BACKTRACE=1

.PHONY: clippy
clippy: ## Runs clippy showing warnings
	cargo clippy --all-targets -- -D warnings

.PHONY: format
format: ## Formats source tree
	cargo fmt --all

.PHONY: test
test: ## Run all tests
	$(BACKTRACE) RUSTFLAGS="-C target-cpu=native" cargo test

.PHONY: test-wasm
test-wasm: ## Run all tests in WASM environment
	$(BACKTRACE) cargo test --target wasm32-wasip1 --no-default-features

.PHONY: coverage
coverage: ## Generates HTML code coverage report, using `cargo-tarpaulin`
	cargo tarpaulin -t 600 --out Html

.PHONY: bench
bench: ## Run all benchmarks
	RUSTFLAGS="-C target-cpu=native" cargo bench --all-features --profile optimized

.PHONY: clean
clean: ## Removes cargo target directory
	cargo clean

.PHONY: example
example: ## Runs example programs demonstrating usage of TurboSHAKE API
	RUSTFLAGS="-C target-cpu=native" cargo run --example turboshake128
	RUSTFLAGS="-C target-cpu=native" cargo run --example turboshake256

.PHONY: example-wasm
example-wasm: ## Runs example programs in WASM environment
	cargo run --example turboshake128 --target wasm32-wasip1 --no-default-features
	cargo run --example turboshake256 --target wasm32-wasip1 --no-default-features
