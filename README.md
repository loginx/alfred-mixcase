# alfred-mixcase

Alfred workflow designed to convert arbitrary strings into mixed case.

Hotkey support to mix MacOS selected text from other applications.

## Installation

Download `MixCase.alfredworkflow` from the [latest release](https://github.com/loginx/alfred-mixcase/releases/latest) and open it. The workflow strips the macOS quarantine attribute from its binary on first run.

## Building

Requires [rustup](https://rustup.rs/) and Xcode Command Line Tools (`lipo`, `strip`). `rust-toolchain.toml` pulls in both macOS targets.

```bash
make alfredworkflow
```

This builds a universal binary (x86_64 + arm64) into `bin/` and packages `MixCase.alfredworkflow`.

## Releasing

Releases are cut by [release-please](https://github.com/googleapis/release-please) from [Conventional Commit](https://www.conventionalcommits.org/) PR titles. Merging its release PR tags the version and attaches the workflow to the GitHub release.

## Usage

- Type `mx` followed by your text to convert it to mixed case
- Use the hotkey (⌘⌥L) to convert selected text
