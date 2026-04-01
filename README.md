# alfred-mixcase

Alfred workflow designed to convert arbitrary strings into mixed case.

Hotkey support to mix MacOS selected text from other applications.

## Building

This workflow is built with Rust. To build the universal binary:

```bash
make build
```

This will create a universal binary (x86_64 + arm64) in the `bin/` directory.

## Requirements

- Rust (install via [rustup](https://rustup.rs/))
- Xcode Command Line Tools (for `lipo`)

## Installation

1. Build the binary: `make build`
2. Open the workflow in Alfred
3. The workflow will automatically handle macOS quarantine attributes

## Usage

- Type `mx` followed by your text to convert it to mixed case
- Use the hotkey (Cmd+L) to convert selected text
