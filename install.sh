#!/bin/bash
set -e

# cc-statusline installer
# Usage: curl -fsSL https://raw.githubusercontent.com/Demwunz/cc-statusline/main/install.sh | bash

REPO="Demwunz/cc-statusline"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"
BINARY_NAME="cc-statusline"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

info() {
    printf "${BLUE}info${NC}: %s\n" "$1"
}

success() {
    printf "${GREEN}success${NC}: %s\n" "$1"
}

warn() {
    printf "${YELLOW}warn${NC}: %s\n" "$1"
}

error() {
    printf "${RED}error${NC}: %s\n" "$1" >&2
    exit 1
}

detect_os() {
    case "$(uname -s)" in
        Linux*)  echo "linux" ;;
        Darwin*) echo "darwin" ;;
        *)       error "Unsupported operating system: $(uname -s)" ;;
    esac
}

detect_arch() {
    case "$(uname -m)" in
        x86_64|amd64)  echo "x86_64" ;;
        arm64|aarch64) echo "arm64" ;;
        *)             error "Unsupported architecture: $(uname -m)" ;;
    esac
}

get_latest_version() {
    curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/'
}

main() {
    info "Installing ${BINARY_NAME}..."

    OS=$(detect_os)
    ARCH=$(detect_arch)
    info "Detected OS: ${OS}, Architecture: ${ARCH}"

    VERSION="${VERSION:-$(get_latest_version)}"
    if [ -z "$VERSION" ]; then
        error "Could not determine latest version. Please set VERSION environment variable."
    fi
    info "Installing version: ${VERSION}"

    ARTIFACT="${BINARY_NAME}-${OS}-${ARCH}"
    DOWNLOAD_URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARTIFACT}"

    info "Downloading from ${DOWNLOAD_URL}..."

    TMP_DIR=$(mktemp -d)
    trap "rm -rf ${TMP_DIR}" EXIT

    if ! curl -fsSL "${DOWNLOAD_URL}" -o "${TMP_DIR}/${BINARY_NAME}"; then
        error "Failed to download ${ARTIFACT}. Check if the release exists for your platform."
    fi

    chmod +x "${TMP_DIR}/${BINARY_NAME}"

    # Check if we can write to INSTALL_DIR
    if [ -w "${INSTALL_DIR}" ]; then
        mv "${TMP_DIR}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
    else
        info "Requesting sudo access to install to ${INSTALL_DIR}..."
        sudo mv "${TMP_DIR}/${BINARY_NAME}" "${INSTALL_DIR}/${BINARY_NAME}"
    fi

    success "Installed ${BINARY_NAME} to ${INSTALL_DIR}/${BINARY_NAME}"

    # Verify installation
    if command -v "${BINARY_NAME}" &> /dev/null; then
        success "Installation complete! Run '${BINARY_NAME} --version' to verify."
    else
        warn "Installed successfully but ${INSTALL_DIR} may not be in your PATH."
        warn "Add 'export PATH=\"${INSTALL_DIR}:\$PATH\"' to your shell profile."
    fi

    echo ""
    info "To use with Claude Code, add to ~/.claude/settings.json:"
    echo ""
    echo '  {'
    echo '    "statusLine": {'
    echo '      "type": "command",'
    echo '      "command": "cc-statusline"'
    echo '    }'
    echo '  }'
    echo ""
}

main "$@"
