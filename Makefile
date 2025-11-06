.PHONY: all test build clean

BIN_DIR := bin
BINARY_NAME := mixcase

all: build

test:
	cargo test

build:
	mkdir -p $(BIN_DIR)
	# Build for amd64
	cargo build --release --target x86_64-apple-darwin
	# Build for arm64
	cargo build --release --target aarch64-apple-darwin
	# Create universal binary
	lipo -create \
		target/x86_64-apple-darwin/release/$(BINARY_NAME) \
		target/aarch64-apple-darwin/release/$(BINARY_NAME) \
		-output $(BIN_DIR)/$(BINARY_NAME)
	# Strip symbols to reduce size
	strip $(BIN_DIR)/$(BINARY_NAME)

clean:
	rm -rf $(BIN_DIR)
	cargo clean

