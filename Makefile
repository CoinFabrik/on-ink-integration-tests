fmt: fmt-rust
fmt-check: fmt-rust-check

fmt-rust:
	@echo "Formatting Rust code..."
	@./scripts/list-cargo-directories.sh | ./scripts/run-cargo-fmt.sh

fmt-rust-check:
	@echo "Checking Rust code formatting..."
	@./scripts/list-cargo-directories.sh | ./scripts/run-cargo-fmt.sh --check
