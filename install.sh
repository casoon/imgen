#!/usr/bin/env sh
set -e

REPO="casoon/imgen"
BIN="imgen"
INSTALL_DIR="${IMGEN_INSTALL_DIR:-/usr/local/bin}"

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Darwin)
    case "$ARCH" in
      arm64)  TARGET="aarch64-apple-darwin" ;;
      x86_64) TARGET="x86_64-apple-darwin" ;;
      *)      echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
    esac
    ;;
  Linux)
    case "$ARCH" in
      aarch64) TARGET="aarch64-unknown-linux-gnu" ;;
      x86_64)  TARGET="x86_64-unknown-linux-gnu" ;;
      *)       echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
    esac
    ;;
  *)
    echo "Unsupported OS: $OS" >&2
    echo "Build from source instead: cargo install --git https://github.com/${REPO}" >&2
    exit 1
    ;;
esac

# Resolve version
if [ -z "$VERSION" ]; then
  VERSION="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed 's/.*"tag_name": *"v\([^"]*\)".*/\1/')"
fi

if [ -z "$VERSION" ]; then
  echo "Could not determine latest version" >&2
  exit 1
fi

ASSET="${BIN}-${TARGET}.tar.gz"
RELEASE_BASE_URL="${IMGEN_RELEASE_BASE_URL:-https://github.com/${REPO}/releases/download/v${VERSION}}"
URL="${RELEASE_BASE_URL}/${ASSET}"
CHECKSUM_URL="${RELEASE_BASE_URL}/SHA256SUMS"

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

echo "Installing imgen v${VERSION} (${TARGET})..."
curl -fsSL "$URL" -o "${TMP}/${ASSET}"
curl -fsSL "$CHECKSUM_URL" -o "${TMP}/SHA256SUMS"
EXPECTED_CHECKSUM="$(awk -v asset="$ASSET" '$2 == asset || $2 == "*" asset { print $1 }' "${TMP}/SHA256SUMS")"
if [ -z "$EXPECTED_CHECKSUM" ]; then
  echo "No checksum published for ${ASSET}" >&2
  exit 1
fi
if command -v sha256sum >/dev/null 2>&1; then
  ACTUAL_CHECKSUM="$(sha256sum "${TMP}/${ASSET}" | awk '{ print $1 }')"
elif command -v shasum >/dev/null 2>&1; then
  ACTUAL_CHECKSUM="$(shasum -a 256 "${TMP}/${ASSET}" | awk '{ print $1 }')"
else
  echo "No SHA-256 verification tool found" >&2
  exit 1
fi
if [ "$ACTUAL_CHECKSUM" != "$EXPECTED_CHECKSUM" ]; then
  echo "Checksum verification failed for ${ASSET}" >&2
  exit 1
fi

tar -xzf "${TMP}/${ASSET}" -C "$TMP"

if [ ! -w "$INSTALL_DIR" ]; then
  echo "Installing to ${INSTALL_DIR} (requires sudo)..."
  sudo install -m 755 "${TMP}/${BIN}" "${INSTALL_DIR}/${BIN}"
else
  install -m 755 "${TMP}/${BIN}" "${INSTALL_DIR}/${BIN}"
fi

echo "Installed: $(${INSTALL_DIR}/${BIN} --version)"
