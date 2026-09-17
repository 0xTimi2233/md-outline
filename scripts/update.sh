#!/usr/bin/env bash
set -euo pipefail

INSTALL_DIR="${HOME}/.local/bin"
mkdir -p "${INSTALL_DIR}"

# Detect OS and architecture
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "${OS}" in
  darwin)
    if [ "${ARCH}" = "arm64" ] || [ "${ARCH}" = "aarch64" ]; then
      ARTIFACT="md-outline-darwin-arm64"
    else
      ARTIFACT="md-outline-darwin-x64"
    fi
    ;;
  linux)
    if [ "${ARCH}" = "arm64" ] || [ "${ARCH}" = "aarch64" ]; then
      ARTIFACT="md-outline-linux-arm64"
    else
      ARTIFACT="md-outline-linux-x64"
    fi
    ;;
  msys*|mingw*|cygwin*)
    ARTIFACT="md-outline-windows-x64.exe"
    ;;
  *)
    echo "错误: 不支持的操作系统架构: ${OS}-${ARCH}" >&2
    exit 1
    ;;
esac

echo "正在从 GitHub Release 获取最新发布产物 (${ARTIFACT})..."

if command -v gh >/dev/null 2>&1; then
  gh release download --pattern "${ARTIFACT}" --dir "${INSTALL_DIR}" --clobber
else
  REPO_URL=$(git config --get remote.origin.url || true)
  if [ -z "${REPO_URL}" ]; then
    echo "错误: 未检测到 git 仓库 remote 配置，无法定位发布地址" >&2
    exit 1
  fi
  REPO_SLUG=$(echo "${REPO_URL}" | sed -E 's/.*[:/]([^/]+\/[^/]+)(\.git)?$/\1/' | sed 's/\.git$//')
  DOWNLOAD_URL="https://github.com/${REPO_SLUG}/releases/latest/download/${ARTIFACT}"
  curl -fsSL "${DOWNLOAD_URL}" -o "${INSTALL_DIR}/${ARTIFACT}"
fi

mv -f "${INSTALL_DIR}/${ARTIFACT}" "${INSTALL_DIR}/md-outline"
chmod +x "${INSTALL_DIR}/md-outline"

echo "本地安装态已成功更新 -> ${INSTALL_DIR}/md-outline"
"${INSTALL_DIR}/md-outline" --version
