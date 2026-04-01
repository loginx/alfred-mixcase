.PHONY: all test build clean alfredworkflow

BIN_DIR := bin
BINARY_NAME := mixcase
WORKFLOW_NAME := MixCase.alfredworkflow

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

alfredworkflow: build
	rm -f $(WORKFLOW_NAME)
	mkdir -p workflow
	cp $(BIN_DIR)/$(BINARY_NAME) workflow/
	cp info.plist workflow/
	# Copy icon if it exists (flickr.png or icon.png)
	if [ -f icon.png ]; then \
		cp icon.png workflow/; \
	elif [ -f flickr.png ]; then \
		cp flickr.png workflow/icon.png; \
	fi
	cd workflow && zip -r ../$(WORKFLOW_NAME) $(BINARY_NAME) info.plist icon.png 2>/dev/null || zip -r ../$(WORKFLOW_NAME) $(BINARY_NAME) info.plist
	rm -rf workflow

clean:
	rm -rf $(BIN_DIR) $(WORKFLOW_NAME)
	cargo clean

