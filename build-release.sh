#!/bin/bash

# Build script for Rusty Asteroid releases
# This script builds the game for multiple platforms

set -e

echo "🚀 Building Rusty Asteroid for distribution..."

# Create releases directory
mkdir -p releases

# Build for current platform (macOS)
echo "📦 Building for macOS (current platform)..."
cargo build --release
cp target/release/rusty_asteroid releases/rusty_asteroid-macos-arm64

# Make executable
chmod +x releases/rusty_asteroid-macos-arm64

echo "✅ Build complete!"
echo ""
echo "📁 Files created in releases/ directory:"
ls -la releases/
echo ""
echo "🎯 To distribute:"
echo "1. Commit and push your changes to GitHub"
echo "2. Create a new release tag: git tag v0.1.0 && git push origin v0.1.0"
echo "3. GitHub Actions will automatically build for all platforms!"
echo "4. Check the Actions tab for build progress"
echo ""
echo "🤖 GitHub Actions will build:"
echo "   - Linux (x86_64)"
echo "   - Windows (x86_64)"
echo "   - macOS Intel (x86_64)"
echo "   - macOS Apple Silicon (ARM64)"
echo ""
echo "📋 Manual builds are still possible - see releases/README.md"
