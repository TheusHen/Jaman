#!/usr/bin/env bash
#
# Jaman Installation Script for Unix-like systems (Linux, macOS)
# 
# Usage: curl -fsSL https://raw.githubusercontent.com/TheusHen/jaman/main/install.sh | bash

set -e

REPO="TheusHen/jaman"
INSTALL_DIR="$HOME/Downloads/jaman"
BIN_NAME="jaman"

# Detect OS and architecture
detect_platform() {
    local os=$(uname -s | tr '[:upper:]' '[:lower:]')
    local arch=$(uname -m)
    
    case "$os" in
        linux*)
            OS="linux"
            ;;
        darwin*)
            OS="macos"
            ;;
        *)
            echo "Unsupported operating system: $os"
            exit 1
            ;;
    esac
    
    case "$arch" in
        x86_64|amd64)
            ARCH="x64"
            ;;
        aarch64|arm64)
            ARCH="arm64"
            ;;
        *)
            echo "Unsupported architecture: $arch"
            exit 1
            ;;
    esac
    
    ASSET_NAME="${BIN_NAME}-${OS}-${ARCH}"
    echo "Detected platform: ${OS} ${ARCH}"
}

# Get the latest release version
get_latest_version() {
    echo "Fetching latest version..."
    LATEST_VERSION=$(curl -s "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"v([^"]+)".*/\1/')
    
    if [ -z "$LATEST_VERSION" ]; then
        echo "Failed to fetch latest version"
        exit 1
    fi
    
    echo "Latest version: v${LATEST_VERSION}"
}

# Download and install
install_jaman() {
    local download_url="https://github.com/${REPO}/releases/download/v${LATEST_VERSION}/${ASSET_NAME}.tar.gz"
    local temp_file="/tmp/${ASSET_NAME}.tar.gz"
    
    echo "Downloading from ${download_url}..."
    curl -fsSL "$download_url" -o "$temp_file"
    
    echo "Creating installation directory: ${INSTALL_DIR}"
    mkdir -p "$INSTALL_DIR"
    
    echo "Extracting..."
    tar -xzf "$temp_file" -C "$INSTALL_DIR"
    
    echo "Making executable..."
    chmod +x "${INSTALL_DIR}/${BIN_NAME}"
    
    echo "Cleaning up..."
    rm "$temp_file"
}

# Add to PATH
setup_path() {
    local shell_config=""
    
    if [ -n "$ZSH_VERSION" ]; then
        shell_config="$HOME/.zshrc"
    elif [ -n "$BASH_VERSION" ]; then
        if [ -f "$HOME/.bashrc" ]; then
            shell_config="$HOME/.bashrc"
        elif [ -f "$HOME/.bash_profile" ]; then
            shell_config="$HOME/.bash_profile"
        fi
    fi
    
    if [ -z "$shell_config" ]; then
        shell_config="$HOME/.profile"
    fi
    
    local path_line="export PATH=\"\$PATH:${INSTALL_DIR}\""
    
    if ! grep -q "$INSTALL_DIR" "$shell_config" 2>/dev/null; then
        echo "" >> "$shell_config"
        echo "# Added by jaman installer" >> "$shell_config"
        echo "$path_line" >> "$shell_config"
        echo "Added to PATH in $shell_config"
    else
        echo "PATH already configured in $shell_config"
    fi
    
    # Add to current session
    export PATH="$PATH:$INSTALL_DIR"
}

# Verify installation
verify_installation() {
    if [ -x "${INSTALL_DIR}/${BIN_NAME}" ]; then
        echo ""
        echo "✓ Jaman installed successfully!"
        echo ""
        echo "Installation location: ${INSTALL_DIR}/${BIN_NAME}"
        echo ""
        echo "To start using jaman, either:"
        echo "  1. Restart your terminal, or"
        echo "  2. Run: source ~/.bashrc (or ~/.zshrc)"
        echo ""
        echo "Then run: jaman --version"
        echo ""
        echo "Get started with: jaman --help"
        return 0
    else
        echo "Installation failed"
        return 1
    fi
}

# Main installation flow
main() {
    echo "=========================================="
    echo "  Jaman Installer"
    echo "=========================================="
    echo ""
    
    detect_platform
    get_latest_version
    install_jaman
    setup_path
    verify_installation
}

main
