#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-}"
if [[ -z "$VERSION" || ! "$VERSION" =~ ^[0-9]+\.[0-9]+\.[0-9]+.*$ ]]; then
  echo "用法: just bump <x.y.z>" >&2
  exit 1
fi

CARGO_TOML="$(dirname "$0")/../Cargo.toml"

# Update version in Cargo.toml [workspace.package] section
sed -i '' -E "s/^version = \"[0-9]+\.[0-9]+\.[0-9]+.*\"/version = \"${VERSION}\"/" "$CARGO_TOML"

echo "版本号已成功同步更新至 ${VERSION}"
