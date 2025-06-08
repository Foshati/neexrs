#!/bin/bash

# Build and test script for neex
set -e

echo "🚀 Building neex..."

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Run tests
echo "🧪 Running tests..."
cargo test --all-features

# Check formatting
echo "📝 Checking code formatting..."
cargo fmt --all -- --check

# Run clippy for linting
echo "🔍 Running clippy..."
cargo clippy --all-targets --all-features -- -D warnings

# Build in release mode
echo "⚡ Building release binary..."
cargo build --release

# Test the binary
echo "🎯 Testing the binary..."
./target/release/neex --help

echo "✅ Build completed successfully!"
echo "Binary location: ./target/release/neex"
echo ""
echo "To install globally:"
echo "  cargo install --path ."
echo ""
echo "To test commands:"
echo "  ./target/release/neex s 'echo hello' 'echo world'"
echo "  ./target/release/neex p 'sleep 1 && echo fast' 'sleep 2 && echo slow'"