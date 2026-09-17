#!/usr/bin/env bash
set -euo pipefail

INSTALL_DIR="${HOME}/.local/bin"
mkdir -p "${INSTALL_DIR}"

OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "${OS}" in
  darwin)
    if [ "${ARCH}" = "arm64" ] || [ "${ARCH}" = "aarch64" ]; then
      TARGET="aarch64-apple-darwin"
    else
      TARGET="x86_64-apple-darwin"
    fi
    ARCHIVE_EXT="tar.xz"
    ;;
  linux)
    if [ "${ARCH}" = "arm64" ] || [ "${ARCH}" = "aarch64" ]; then
      TARGET="aarch64-unknown-linux-gnu"
    else
      TARGET="x86_64-unknown-linux-gnu"
    fi
    ARCHIVE_EXT="tar.xz"
    ;;
  msys*|mingw*|cygwin*)
    TARGET="x86_64-pc-windows-msvc"
    ARCHIVE_EXT="zip"
    ;;
  *)
    echo "错误: 不支持的操作系统架构: ${OS}-${ARCH}" >&2
    exit 1
    ;;
esac

ARTIFACT="md-outline-${TARGET}.${ARCHIVE_EXT}"
TMP_DIR="$(mktemp -d)"
trap 'rm -rf "${TMP_DIR}"' EXIT

echo "正在从 GitHub Release 获取最新发布产物 (${ARTIFACT})..."

if command -v gh >/dev/null 2>&1; then
  gh release download --pattern "${ARTIFACT}" --dir "${TMP_DIR}" --clobber
else
  REPO_URL=$(git config --get remote.origin.url || true)
  if [ -z "${REPO_URL}" ]; then
    echo "错误: 未检测到 git 仓库 remote 配置，无法定位发布地址" >&2
    exit 1
  fi
  REPO_SLUG=$(echo "${REPO_URL}" | sed -E 's/.*[:/]([^/]+\/[^/]+)(\.git)?$/\1/' | sed 's/\.git$//')
  DOWNLOAD_URL="https://github.com/${REPO_SLUG}/releases/latest/download/${ARTIFACT}"
  curl -fsSL "${DOWNLOAD_URL}" -o "${TMP_DIR}/${ARTIFACT}"
fi

if [ "${ARCHIVE_EXT}" = "tar.xz" ]; then
  tar -xf "${TMP_DIR}/${ARTIFACT}" -C "${TMP_DIR}"
  find "${TMP_DIR}" -type f -name "md-outline" -exec mv -f {} "${INSTALL_DIR}/md-outline" \;
elif [ "${ARCHIVE_EXT}" = "zip" ]; then
  unzip -q -o "${TMP_DIR}/${ARTIFACT}" -d "${TMP_DIR}"
  find "${TMP_DIR}" -type f -name "md-outline.exe" -exec mv -f {} "${INSTALL_DIR}/md-outline.exe" \;
fi

chmod +x "${INSTALL_DIR}/md-outline"

echo "本地安装态已成功更新 -> ${INSTALL_DIR}/md-outline"
"${INSTALL_DIR}/md-outline" --version
