#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-}"
if [[ -z "$VERSION" || ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+.*$ ]]; then
  echo "用法: just bump <x.y.z>" >&2
  exit 1
fi

ROOT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
CARGO_TOML="${ROOT_DIR}/Cargo.toml"

# Update version in Cargo.toml [workspace.package] section
sed -i '' -E "s/^version = \"[0-9]+\.[0-9]+\.[0-9]+.*\"/version = \"${VERSION}\"/" "$CARGO_TOML"

# Update Cargo.lock to match new package version
(cd "${ROOT_DIR}" && cargo check --quiet)

echo "版本号已成功同步更新至 ${VERSION}（Cargo.lock 已同步刷新）"
