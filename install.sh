#!/bin/bash

set -e

#https://github.com/jamshid-elmurodov/rfetch/releases/download/1.0.0/rfetch-darwin-arm64

URL_PREFIX="https://github.com/jamshid-elmurodov/rfetch/releases/download/1.0.0"
INSTALL_DIR=${INSTALL_DIR:-/usr/local/bin}
CONFIG_DIR="$HOME/.config/rfetch"

case "$(uname -sm)" in
  "Darwin x86_64") FILENAME="rfetch-darwin-amd64" ;;
  "Darwin arm64") FILENAME="rfetch-darwin-arm64" ;;
  "Linux x86_64") FILENAME="rfetch-linux-amd64" ;;
  "Linux i686") FILENAME="rfetch-linux-386" ;;
  "Linux armv7l") FILENAME="rfetch-linux-arm" ;;
  "Linux aarch64") FILENAME="rfetch-linux-arm64" ;;
  *) echo "Unknown architecture: $(uname -sm) is not supported." >&2; exit 1 ;;
esac

echo "Downloading $FILENAME from GitHub..."
if ! curl -sSLf "$URL_PREFIX/$FILENAME" -o "$INSTALL_DIR/rfetch"; then
  echo "Failed to write to $INSTALL_DIR; please try running with sudo." >&2
  exit 1
fi

if ! chmod +x "$INSTALL_DIR/rfetch"; then
  echo "Failed to grant execution permissions to $INSTALL_DIR/rfetch." >&2
  exit 1
fi

echo "Creating configuration directory: $CONFIG_DIR"
mkdir -p "$CONFIG_DIR"

echo "Downloading config.toml..."
if ! curl -sSLf "https://github.com/jamshid-elmurodov/rfetch/releases/download/1.0.0/config.toml" -o "$CONFIG_DIR/config.toml"; then
  echo "Failed to download config.toml." >&2
  exit 1
fi

echo "Downloading default.txt..."
if ! curl -sSLf "https://github.com/jamshid-elmurodov/rfetch/releases/download/1.0.0/default.txt" -o "$CONFIG_DIR/default.txt"; then
  echo "Failed to download default.txt." >&2
  exit 1
fi

echo "rfetch has been successfully installed and configured!"