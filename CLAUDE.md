# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
This is a simple Rust project named "bccl" using Rust edition 2024. The project is currently in its initial state with a basic "Hello, world!" program.

## Commands

### Build and Development
- `cargo build` - Compile the project
- `cargo build --release` - Build optimized release version
- `cargo run` - Build and run the main binary
- `cargo check` - Quick compile check without producing binaries

### Testing
- `cargo test` - Run all tests
- `cargo test <test_name>` - Run specific test containing the name
- `cargo test --no-run` - Compile tests but don't run them

### Other Useful Commands
- `cargo clean` - Remove build artifacts from target directory
- `cargo doc` - Generate documentation
- `cargo clippy` - Run Rust linter (if clippy is installed)
- `cargo fmt` - Format code (if rustfmt is installed)

## Project Structure
- `src/main.rs` - Main entry point containing the basic "Hello, world!" program
- `Cargo.toml` - Project manifest with metadata and dependencies
- `Cargo.lock` - Dependency lock file (auto-generated)
- `target/` - Build output directory (auto-generated, excluded from git)

## Architecture Notes
The project is currently minimal with no external dependencies or complex architecture. Future development should follow standard Rust project conventions and consider adding appropriate modules in the `src/` directory as the codebase grows.