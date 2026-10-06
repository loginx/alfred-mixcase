.PHONY: all test build clean alfredworkflow

BIN := bin/mixcase
WORKFLOW := MixCase.alfredworkflow
TARGETS := x86_64-apple-darwin aarch64-apple-darwin

all: build

test:
	cargo test

build:
	cargo build --release $(addprefix --target ,$(TARGETS))
	mkdir -p $(dir $(BIN))
	lipo -create $(foreach t,$(TARGETS),target/$(t)/release/mixcase) -output $(BIN)
	strip $(BIN)

alfredworkflow: build
	rm -f $(WORKFLOW)
	zip -qj $(WORKFLOW) $(BIN) info.plist icon.png

clean:
	rm -rf $(dir $(BIN)) $(WORKFLOW)
	cargo clean
