#!/usr/bin/env bash
set -euo pipefail

echo "正在从最新 GitHub Release 获取官方安装器并更新本地二进制..."
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/0xTimi2233/md-outline/releases/latest/download/md-outline-installer.sh | sh

echo "本地安装态已成功更新："
"${HOME}/.cargo/bin/md-outline" --version
