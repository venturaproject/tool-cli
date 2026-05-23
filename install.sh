#!/bin/sh
set -e

REPO="venturaproject/tooler"
BIN_NAME="tooler"
INSTALL_DIR="/usr/local/bin"

# ── detect OS ────────────────────────────────────────────────────────────────
OS="$(uname -s)"
case "$OS" in
  Linux)  OS="linux"  ;;
  Darwin) OS="macos"  ;;
  *)
    echo "error: unsupported OS: $OS" >&2
    exit 1
    ;;
esac

# ── detect architecture ───────────────────────────────────────────────────────
ARCH="$(uname -m)"
case "$ARCH" in
  x86_64 | amd64) ARCH="x86_64"  ;;
  aarch64 | arm64) ARCH="aarch64" ;;
  *)
    echo "error: unsupported architecture: $ARCH" >&2
    exit 1
    ;;
esac

# ── resolve latest version ────────────────────────────────────────────────────
if [ -z "$TOOLER_VERSION" ]; then
  TOOLER_VERSION="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" \
    | grep '"tag_name"' | sed 's/.*"tag_name": *"\([^"]*\)".*/\1/')"
fi

if [ -z "$TOOLER_VERSION" ]; then
  echo "error: could not determine latest version. Set TOOLER_VERSION manually." >&2
  exit 1
fi

ARTIFACT="${BIN_NAME}-${OS}-${ARCH}"
URL="https://github.com/${REPO}/releases/download/${TOOLER_VERSION}/${ARTIFACT}"

echo "Installing tooler ${TOOLER_VERSION} (${OS}/${ARCH})..."

# ── download ──────────────────────────────────────────────────────────────────
TMP="$(mktemp)"
if ! curl -fsSL "$URL" -o "$TMP"; then
  echo "error: download failed from $URL" >&2
  rm -f "$TMP"
  exit 1
fi
chmod +x "$TMP"

# ── install ───────────────────────────────────────────────────────────────────
# Try /usr/local/bin first; fall back to ~/.local/bin if no sudo access
if [ -w "$INSTALL_DIR" ] || sudo -n true 2>/dev/null; then
  sudo mv "$TMP" "${INSTALL_DIR}/${BIN_NAME}"
  echo "Installed to ${INSTALL_DIR}/${BIN_NAME}"
else
  LOCAL_BIN="$HOME/.local/bin"
  mkdir -p "$LOCAL_BIN"
  mv "$TMP" "${LOCAL_BIN}/${BIN_NAME}"
  echo "Installed to ${LOCAL_BIN}/${BIN_NAME}"
  echo ""
  echo "Make sure ${LOCAL_BIN} is in your PATH:"
  echo '  export PATH="$HOME/.local/bin:$PATH"'
fi

echo ""
tooler --version 2>/dev/null && echo "Done. Run: tooler --help" || echo "Done. Open a new terminal and run: tooler --help"
