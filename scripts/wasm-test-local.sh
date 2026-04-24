#!/usr/bin/env bash

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TMP_ROOT="${TMPDIR:-/tmp}"
CACHE_ROOT="${XDG_CACHE_HOME:-$HOME/.cache}/sql-intelliscan"

# Local wasm test artifacts on the external drive can become invalid.
# Keep the target dir on the local filesystem without affecting CI.
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-${TMP_ROOT%/}/sql-intelliscan-wasm-target}"

cd "$ROOT_DIR"

echo "Using local wasm target dir: $CARGO_TARGET_DIR"

find_chrome_bin() {
  local candidate

  for candidate in \
    "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome" \
    "$(command -v google-chrome 2>/dev/null || true)" \
    "$(command -v chromium 2>/dev/null || true)" \
    "$(command -v chromium-browser 2>/dev/null || true)"
  do
    if [[ -n "$candidate" && -x "$candidate" ]]; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done

  return 1
}

chrome_major_version() {
  local chrome_bin version
  chrome_bin="$(find_chrome_bin)" || return 1
  version="$("$chrome_bin" --version)"
  printf '%s\n' "$version" | sed -E 's/.* ([0-9]+)\..*/\1/'
}

chromedriver_major_version() {
  local driver_bin="${1:-}"
  if [[ -z "$driver_bin" || ! -x "$driver_bin" ]]; then
    return 1
  fi
  "$driver_bin" --version | sed -E 's/.* ([0-9]+)\..*/\1/'
}

download_matching_chromedriver() {
  local chrome_major="$1"
  local platform latest_release archive_url cache_dir zip_path
  local uname_s uname_m

  uname_s="$(uname -s)"
  uname_m="$(uname -m)"

  case "$uname_s" in
    Darwin)
      case "$uname_m" in
        arm64) platform="mac-arm64" ;;
        x86_64) platform="mac-x64" ;;
        *) echo "Unsupported macOS architecture: $uname_m" >&2; return 1 ;;
      esac
      ;;
    Linux)
      case "$uname_m" in
        x86_64) platform="linux64" ;;
        aarch64|arm64) platform="linux-arm64" ;;
        *) echo "Unsupported Linux architecture: $uname_m" >&2; return 1 ;;
      esac
      ;;
    *)
      echo "Unsupported OS for local chromedriver download: $uname_s" >&2
      return 1
      ;;
  esac

  latest_release="$(curl -fsSL "https://googlechromelabs.github.io/chrome-for-testing/LATEST_RELEASE_${chrome_major}")"
  cache_dir="${CACHE_ROOT}/chromedriver/${latest_release}-${platform}"

  if [[ ! -x "${cache_dir}/chromedriver-${platform}/chromedriver" ]]; then
    mkdir -p "$cache_dir"
    zip_path="${cache_dir}/chromedriver.zip"
    archive_url="https://storage.googleapis.com/chrome-for-testing-public/${latest_release}/${platform}/chromedriver-${platform}.zip"
    echo "Downloading matching chromedriver ${latest_release} for ${platform}" >&2
    curl -fsSL "$archive_url" -o "$zip_path"
    unzip -oq "$zip_path" -d "$cache_dir"
  fi

  printf '%s\n' "${cache_dir}/chromedriver-${platform}/chromedriver"
}

resolve_chromedriver() {
  local chrome_major system_driver system_major

  chrome_major="$(chrome_major_version)" || {
    echo "Chrome not found. Install Google Chrome or set CHROMEDRIVER manually." >&2
    return 1
  }

  if [[ -n "${CHROMEDRIVER:-}" && -x "${CHROMEDRIVER}" ]]; then
    system_driver="$CHROMEDRIVER"
  elif command -v chromedriver >/dev/null 2>&1; then
    system_driver="$(command -v chromedriver)"
  else
    system_driver=""
  fi

  system_major="$(chromedriver_major_version "$system_driver" || true)"
  if [[ -n "$system_driver" && "$system_major" == "$chrome_major" ]]; then
    printf '%s\n' "$system_driver"
    return 0
  fi

  if [[ -n "$system_driver" ]]; then
    echo "Ignoring system chromedriver ${system_major:-unknown}; Chrome major is ${chrome_major}" >&2
  fi

  download_matching_chromedriver "$chrome_major"
}

MATCHED_CHROMEDRIVER="$(resolve_chromedriver)"
echo "Using chromedriver: $MATCHED_CHROMEDRIVER"

unset CHROMEDRIVER

exec wasm-pack test --headless --chrome --chromedriver "$MATCHED_CHROMEDRIVER" -- --test frontend "$@"
