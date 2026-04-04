.PHONY: setup clean fmt-check fmt clippy clippy-release check check-release build build-release test test-release run run-debug ci doc help

# Setup development environment
setup:
	rustup component add rustfmt clippy
	cargo fetch
	lefthook install

# Cleanup compilation outputs
clean:
	cargo clean

# Check the code format
fmt-check:
	cargo fmt --all -- --check
# Format the code
fmt:
	cargo fmt --all

# Run rust clippy with debug profile
clippy:
	cargo clippy --all --all-targets -- -D warnings
# Run rust clippy with release profile
clippy-release:
	cargo clippy --release --all --all-targets -- -D warnings

# Check code with debug profile
check:
	cargo check
# Check code with release profile
check-release:
	cargo check --release

# Build all binaries with debug profile
build:
	cargo build
# Build all binaries with release profile
build-release:
	cargo build --release

# Run all unit tests with debug profile
test:
	cargo test --all
# Run all unit tests with release profile
test-release:
	cargo test --release --all

# Run the game
run:
	cargo run

# Run the game with debug UI
run-debug:
	cargo run --features debug

# Run all CI checks (fmt, clippy, test, build)
ci: fmt clippy test build

# Generate documentation
doc:
	cargo doc --no-deps --open

# Show help
help:
	@echo ''
	@echo 'Usage:'
	@echo ' make [target]'
	@echo ''
	@echo 'Targets:'
	@awk '/^[a-zA-Z\-\_0-9]+:/ { \
	helpMessage = match(lastLine, /^# (.*)/); \
		if (helpMessage) { \
			helpCommand = substr($$1, 0, index($$1, ":")); \
			helpMessage = substr(lastLine, RSTART + 2, RLENGTH); \
			printf "\033[36m%-30s\033[0m %s\n", helpCommand,helpMessage; \
		} \
	} \
	{ lastLine = $$0 }' $(MAKEFILE_LIST)

.DEFAULT_GOAL := help
