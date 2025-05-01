#!/bin/bash

set -e

INSTALL_DIR="$HOME/.repos/bin"

OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin) OS="apple-darwin" ;;
  Linux) OS="unknown-linux-gnu" ;;
  *) echo "Unsupported OS: $OS" && exit 1 ;;
esac

case "$ARCH" in
  x86_64) ARCH="x86_64" ;;
  arm64 | aarch64) ARCH="aarch64" ;;
  *) echo "Unsupported architecture: $ARCH" && exit 1 ;;
esac

echo "Fetching latest release from GitHub..."
LATEST_VERSION=$(curl -s "https://api.github.com/repos/augustoccesar/repos/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

FILENAME="repos-${LATEST_VERSION}-${ARCH}-${OS}.tar.gz"
URL="https://github.com/augustoccesar/repos/releases/download/$LATEST_VERSION/$FILENAME"

echo "Downloading $FILENAME from $URL..."

mkdir -p "$INSTALL_DIR"
curl -sSL "$URL" | tar -xz -C "$INSTALL_DIR"

echo "Installed in $INSTALL_DIR"

CURRENT_SHELL="$(basename "$SHELL")"

echo ""

if [ "$CURRENT_SHELL" = "fish" ]; then
  echo "To use 'repos' globally, run:"
  echo "  echo 'fish_add_path $INSTALL_DIR' >> $HOME/.config/fish/config.fish"
  echo "And if you want to use the shortcut functions, run:"
  echo "  echo 'repos activate fish | source' >> $HOME/.config/fish/config.fish"
else
  echo "To use 'repos' globally, add this to your shell config:"
  echo "  export PATH=\"\$PATH:$INSTALL_DIR\""
fi
