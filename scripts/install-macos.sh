#!/bin/bash
# QuickTransform - macOS Installation Script
# LAZYFROG-kindware.dev | MIT License

set -e

echo "============================================="
echo "  QuickTransform - Installation"
echo "  LAZYFROG-kindware.dev"
echo "============================================="
echo

INSTALL_DIR="/usr/local/bin"

# Check for sudo if installing to /usr/local/bin
if [ ! -w "$INSTALL_DIR" ]; then
    echo "Note: Installing to $INSTALL_DIR requires sudo."
    SUDO="sudo"
else
    SUDO=""
fi

# Install CLI
if [ -f "target/release/qt" ]; then
    $SUDO install -m 755 target/release/qt "$INSTALL_DIR/"
    echo "[OK] Installed qt to $INSTALL_DIR"
else
    echo "[SKIP] qt binary not found (run: cargo build --release)"
fi

# Install GUI
if [ -f "target/release/qt-gui" ]; then
    $SUDO install -m 755 target/release/qt-gui "$INSTALL_DIR/"
    echo "[OK] Installed qt-gui to $INSTALL_DIR"
else
    echo "[SKIP] qt-gui binary not found (run: cargo build --release --features gui)"
fi

echo
echo "Installation complete."
echo "Run 'qt --help' or 'qt-gui' to get started."
