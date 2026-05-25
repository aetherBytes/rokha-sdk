#!/usr/bin/env sh
# Rokha CLI installer — fetches the matching `ro` binary from GitHub Releases.
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/aetherBytes/rokha-sdk/main/scripts/install.sh | sh
#
# Env overrides:
#   RO_VERSION    pin a specific version tag (default: latest)
#   RO_REPO       owner/repo to install from (default: aetherBytes/rokha-sdk)
#   RO_INSTALL_DIR  install prefix (default: ~/.local/bin if writable, else /usr/local/bin)

set -eu

RO_REPO="${RO_REPO:-aetherBytes/rokha-sdk}"
RO_VERSION="${RO_VERSION:-}"

info()  { printf '\033[1;36m==>\033[0m %s\n' "$*" >&2; }
warn()  { printf '\033[1;33m!!\033[0m %s\n' "$*" >&2; }
fail()  { printf '\033[1;31mxx\033[0m %s\n' "$*" >&2; exit 1; }

need() {
  command -v "$1" >/dev/null 2>&1 || fail "required command not found: $1"
}

need uname
need tar

if command -v curl >/dev/null 2>&1; then
  DL='curl -fsSL'
elif command -v wget >/dev/null 2>&1; then
  DL='wget -qO-'
else
  fail "need curl or wget on PATH"
fi

os=$(uname -s)
arch=$(uname -m)

case "$os" in
  Darwin)
    case "$arch" in
      arm64|aarch64) target="aarch64-apple-darwin" ;;
      x86_64)        target="x86_64-apple-darwin" ;;
      *) fail "unsupported macOS arch: $arch" ;;
    esac
    ;;
  Linux)
    case "$arch" in
      x86_64|amd64)  target="x86_64-unknown-linux-gnu" ;;
      aarch64|arm64) target="aarch64-unknown-linux-gnu" ;;
      *) fail "unsupported Linux arch: $arch" ;;
    esac
    ;;
  *)
    fail "unsupported OS: $os (this installer covers macOS + Linux; Windows users: cargo install rokha-cli)"
    ;;
esac

if [ -z "$RO_VERSION" ]; then
  info "resolving latest release from $RO_REPO ..."
  RO_VERSION=$($DL "https://api.github.com/repos/$RO_REPO/releases/latest" \
    | sed -n 's/.*"tag_name": *"\([^"]*\)".*/\1/p' \
    | head -n1)
  [ -n "$RO_VERSION" ] || fail "could not resolve latest release tag"
fi

version_no_prefix="${RO_VERSION#cli-v}"
asset="ro-${version_no_prefix}-${target}.tar.gz"
url="https://github.com/$RO_REPO/releases/download/$RO_VERSION/$asset"

info "downloading $asset"
tmp=$(mktemp -d 2>/dev/null || mktemp -d -t ro-install)
trap 'rm -rf "$tmp"' EXIT

$DL "$url" > "$tmp/$asset" || fail "download failed: $url"
$DL "$url.sha256" > "$tmp/$asset.sha256" 2>/dev/null || warn "no .sha256 alongside asset — skipping checksum verify"

if [ -s "$tmp/$asset.sha256" ]; then
  info "verifying checksum"
  (cd "$tmp" && shasum -a 256 -c "$asset.sha256" >/dev/null 2>&1) \
    || (cd "$tmp" && sha256sum -c "$asset.sha256" >/dev/null 2>&1) \
    || fail "checksum mismatch"
fi

info "unpacking"
tar -xzf "$tmp/$asset" -C "$tmp"

# Resolve install dir
if [ -n "${RO_INSTALL_DIR:-}" ]; then
  dest="$RO_INSTALL_DIR"
elif [ -w "$HOME/.local/bin" ] 2>/dev/null || mkdir -p "$HOME/.local/bin" 2>/dev/null; then
  dest="$HOME/.local/bin"
elif [ -w /usr/local/bin ]; then
  dest="/usr/local/bin"
else
  dest="$HOME/.local/bin"
  mkdir -p "$dest"
fi

mv "$tmp/ro-${version_no_prefix}-${target}/ro" "$dest/ro"
chmod +x "$dest/ro"

info "installed: $dest/ro"

# PATH hint
case ":$PATH:" in
  *":$dest:"*) ;;
  *) warn "$dest is not on your PATH. Add this to your shell rc:"
     printf '     export PATH="%s:$PATH"\n' "$dest" >&2 ;;
esac

info "verify: $("$dest/ro" version 2>/dev/null || echo 'run: ro version')"
