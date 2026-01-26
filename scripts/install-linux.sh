#!/bin/bash
# QuickTransform - Linux Installation Script
# LAZYFROG-kindware.dev | MIT License

set -e

echo "╔═══════════════════════════════════════════╗"
echo "║  QuickTransform - Installation            ║"
echo "║  LAZYFROG-kindware.dev                    ║"
echo "╚═══════════════════════════════════════════╝"
echo

# Check if running as root
if [ "$EUID" -ne 0 ]; then
    INSTALL_DIR="$HOME/.local/bin"
    DESKTOP_DIR="$HOME/.local/share/applications"
    ICON_DIR="$HOME/.local/share/icons"
else
    INSTALL_DIR="/usr/local/bin"
    DESKTOP_DIR="/usr/share/applications"
    ICON_DIR="/usr/share/icons/hicolor/256x256/apps"
fi

# Create directories
mkdir -p "$INSTALL_DIR"
mkdir -p "$DESKTOP_DIR"
mkdir -p "$ICON_DIR"

# Copy binaries
if [ -f "target/release/qt" ]; then
    cp target/release/qt "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/qt"
    echo "✓ Installed qt CLI to $INSTALL_DIR"
fi

if [ -f "target/release/qt-gui" ]; then
    cp target/release/qt-gui "$INSTALL_DIR/"
    chmod +x "$INSTALL_DIR/qt-gui"
    echo "✓ Installed qt-gui to $INSTALL_DIR"
fi

# Copy desktop file
if [ -f "assets/quicktransform.desktop" ]; then
    sed "s|Exec=qt-gui|Exec=$INSTALL_DIR/qt-gui|" assets/quicktransform.desktop > "$DESKTOP_DIR/quicktransform.desktop"
    echo "✓ Installed desktop entry"
fi

# Copy icon
if [ -f "assets/lazyfrog-kindware-background.png" ]; then
    cp assets/lazyfrog-kindware-background.png "$ICON_DIR/quicktransform.png"
    echo "✓ Installed icon"
fi

echo
echo "Installation complete!"
echo "Run 'qt --help' or launch 'QuickTransform' from your apps menu."
