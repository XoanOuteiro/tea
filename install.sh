#!/usr/bin/env bash
set -e

echo "🔧 Installing teashop..."

echo "🐳 Creating base brew..."
chmod +x init.sh && ./init.sh

echo "🦀 Compiling Rust binary..."
# Change to teashop directory and build
cd teashop
cargo build --release
cd ..

echo "🔧 Setting executable permissions..."
chmod +x teashop/target/release/teashop

# Copy binary to /usr/local/bin (requires sudo)
INSTALL_PATH="/usr/local/bin/teashop"
echo "📦 Copying binary to $INSTALL_PATH"
sudo cp teashop/target/release/teashop "$INSTALL_PATH"

# Copy flavors and other assets to ~/.tea
echo "📁 Copying tea assets to ~/.tea"
mkdir -p ~/.tea
cp -r flavors misc Dockerfile.base ~/.tea/

echo "✅ teashop installed successfully!"
echo "ℹ️  Run with: teashop flavors | teashop brew <flavor> | teashop drink <flavor>"
echo "🧹 Thanks for running, you can delete this folder now!"
