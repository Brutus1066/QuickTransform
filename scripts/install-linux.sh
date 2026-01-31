#!/bin/bash
# QuickTransform - Linux Installation Script
# LAZYFROG-kindware.dev | MIT License

set -e

echo "============================================="
echo "  QuickTransform - Installation"
echo "  LAZYFROG-kindware.dev"
echo "============================================="
echo

# Detect install directories
if [ "$EUID" -eq 0 ]; then
    INSTALL_DIR="/usr/local/bin"
    DESKTOP_DIR="/usr/share/applications"
    ICON_DIR="/usr/share/icons/hicolor/256x256/apps"
else
    INSTALL_DIR="$HOME/.local/bin"
    DESKTOP_DIR="$HOME/.local/share/applications"
    ICON_DIR="$HOME/.local/share/icons/hicolor/256x256/apps"
fi

# Create directories
mkdir -p "$INSTALL_DIR"
mkdir -p "$DESKTOP_DIR"
mkdir -p "$ICON_DIR"

# Install CLI
if [ -f "target/release/qt" ]; then
    install -m 755 target/release/qt "$INSTALL_DIR/"
    echo "[OK] Installed qt to $INSTALL_DIR"
else
    echo "[SKIP] qt binary not found (run: cargo build --release)"
fi

# Install GUI
if [ -f "target/release/qt-gui" ]; then
    install -m 755 target/release/qt-gui "$INSTALL_DIR/"
    echo "[OK] Installed qt-gui to $INSTALL_DIR"
else
    echo "[SKIP] qt-gui binary not found (run: cargo build --release --features gui)"
fi

# Install icon
if [ -f "assets/lazyfrog-kindware-background.png" ]; then
    install -m 644 assets/lazyfrog-kindware-background.png "$ICON_DIR/quicktransform.png"
    echo "[OK] Installed icon"
fi

# Install desktop entry
if [ -f "assets/quicktransform.desktop" ]; then
    sed "s|Exec=qt-gui|Exec=$INSTALL_DIR/qt-gui|" assets/quicktransform.desktop > "$DESKTOP_DIR/quicktransform.desktop"
    chmod 644 "$DESKTOP_DIR/quicktransform.desktop"
    echo "[OK] Installed desktop entry"
    
    # Update desktop database if available
    if command -v update-desktop-database &> /dev/null; then
        update-desktop-database "$DESKTOP_DIR" 2>/dev/null || true
    fi
fi

# Check PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo
    echo "Note: Add $INSTALL_DIR to your PATH:"
    echo "  echo 'export PATH=\"\$PATH:$INSTALL_DIR\"' >> ~/.bashrc"
    echo "  source ~/.bashrc"
fi

echo
echo "Installation complete."
echo "Run 'qt --help' or search for 'QuickTransform' in your apps menu."
