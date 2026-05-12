#!/usr/bin/env bash
set -euo pipefail

REPO="iyulab/declart"
BINARY="declart"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

detect_target() {
    local os arch
    os="$(uname -s)"
    arch="$(uname -m)"

    case "$os" in
        Linux)
            case "$arch" in
                x86_64) echo "x86_64-unknown-linux-musl" ;;
                *) echo "Unsupported Linux architecture: $arch" >&2; exit 1 ;;
            esac
            ;;
        Darwin)
            case "$arch" in
                x86_64) echo "x86_64-apple-darwin" ;;
                arm64)  echo "aarch64-apple-darwin" ;;
                *) echo "Unsupported macOS architecture: $arch" >&2; exit 1 ;;
            esac
            ;;
        *) echo "Unsupported OS: $os" >&2; exit 1 ;;
    esac
}

fetch_latest_version() {
    curl -fsSL "https://api.github.com/repos/$REPO/releases/latest" \
        | grep '"tag_name"' \
        | sed -E 's/.*"tag_name": "([^"]+)".*/\1/'
}

main() {
    local version="${1:-}"
    local target
    target="$(detect_target)"

    if [ -z "$version" ]; then
        echo "Fetching latest release..."
        version="$(fetch_latest_version)"
    fi

    local filename="declart-${version}-${target}.tar.gz"
    local url="https://github.com/$REPO/releases/download/$version/$filename"

    echo "Installing declart $version for $target..."
    echo "Downloading: $url"

    mkdir -p "$INSTALL_DIR"
    curl -fsSL "$url" | tar -xz -C "$INSTALL_DIR" "$BINARY"
    chmod +x "$INSTALL_DIR/$BINARY"

    echo "Installed to $INSTALL_DIR/$BINARY"

    if ! echo "$PATH" | tr ':' '\n' | grep -qx "$INSTALL_DIR"; then
        echo ""
        echo "Add $INSTALL_DIR to your PATH:"
        echo "  export PATH=\"\$HOME/.local/bin:\$PATH\""
    fi

    "$INSTALL_DIR/$BINARY" --version
}

main "$@"
