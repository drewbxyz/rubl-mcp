#!/usr/bin/env bash
set -euo pipefail

# Build MCPB bundle for Rubl - eBird API MCP Server
# This script creates a .mcpb file (ZIP archive) ready for distribution

BUNDLE_NAME="rubl.mcpb"
BUNDLE_DIR="mcpb-bundle"

echo "🦀 Building Rubl MCPB Bundle..."
echo

# Step 1: Build the Rust binary in release mode
echo "Step 1/4: Building Rust binary in release mode..."
cargo build --release
echo "✓ Binary built successfully"
echo

# Step 2: Create bundle directory structure
echo "Step 2/4: Creating bundle directory structure..."
rm -rf "$BUNDLE_DIR"
mkdir -p "$BUNDLE_DIR/server"
echo "✓ Bundle directory created"
echo

# Step 3: Copy files to bundle
echo "Step 3/4: Copying files to bundle..."
cp manifest.json "$BUNDLE_DIR/"
cp target/release/rubl "$BUNDLE_DIR/server/"
chmod +x "$BUNDLE_DIR/server/rubl"
echo "✓ Files copied"
echo

# Step 4: Create .mcpb ZIP archive
echo "Step 4/4: Creating $BUNDLE_NAME archive..."
cd "$BUNDLE_DIR"
zip -r "../$BUNDLE_NAME" . -x "*.DS_Store"
cd ..
echo "✓ Bundle created"
echo

# Verify bundle contents
echo "📦 Bundle contents:"
unzip -l "$BUNDLE_NAME"
echo

# Display bundle info
BUNDLE_SIZE=$(ls -lh "$BUNDLE_NAME" | awk '{print $5}')
echo "✅ Success! Created $BUNDLE_NAME ($BUNDLE_SIZE)"
echo
echo "To install:"
echo "  1. Open $BUNDLE_NAME with Claude for macOS/Windows"
echo "  2. Configure your eBird API key when prompted"
echo "  3. Get a free key at: https://ebird.org/api/keygen"
echo
echo "Or manually test with:"
echo "  EBIRD_API_KEY=your-key $BUNDLE_DIR/server/rubl"
