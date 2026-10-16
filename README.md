# md-outline

[![Crates.io](https://img.shields.io/crates/v/md-outline.svg)](https://crates.io/crates/md-outline)
[![CI](https://github.com/0xTimi2233/md-outline/actions/workflows/ci.yml/badge.svg)](https://github.com/0xTimi2233/md-outline/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

基于 CommonMark 语法树的高性能 Markdown 大纲提取工具，提取具备行号与层级连线的结构树。

## 背景

使用正则表达式提取 Markdown 标题时，代码围栏内部的注释（如 `# TODO`）以及多行 Setext 标题极易引起误判。`md-outline` 采用流式语法分析器解析，过滤代码块干扰，提供行号，为快速检索与文档导航消除噪音。

## 安装

### 一键自动安装（推荐）

#### macOS 与 Linux

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/0xTimi2233/md-outline/releases/latest/download/md-outline-installer.sh | sh
```

#### Windows (PowerShell)

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/0xTimi2233/md-outline/releases/latest/download/md-outline-installer.ps1 | iex"
```

### 通过 cargo-binstall 极速安装

无需本地编译，直接拉取预编译二进制：

```bash
cargo binstall md-outline
```

### 通过 Cargo 源码编译安装

```bash
cargo install md-outline
```

## 使用方法

```bash
# 默认提取：行号搭配 Markdown 原生标题形态
md-outline README.md

# 以经典 Unicode 树状图展示大纲
md-outline -f tree README.md

# 输出为结构化 JSON 格式
md-outline -f json README.md

# 仅查看二级及以上的大纲层级
md-outline -d 2 README.md

# 通过 Unix 管道读取标准输入
cat large-doc.md | md-outline
```

### 实际输出效果

**默认模式（`-f markdown`）**：

```text
1:   # md-outline
9:   ## 背景
13:  ## 安装
15:  ### 一键自动安装（推荐）
29:  ### 通过 cargo-binstall 极速安装
37:  ### 通过 Cargo 源码编译安装
43:  ## 使用方法
62:  ### 实际输出效果
96:  ## 核心特性
107: ## 进阶与协作
114: ## 许可证
```

**经典树形模式（`-f tree`）**：

```text
1:   md-outline
9:   ├── 背景
13:  ├── 安装
15:  │   ├── 一键自动安装（推荐）
29:  │   ├── 通过 cargo-binstall 极速安装
37:  │   └── 通过 Cargo 源码编译安装
43:  ├── 使用方法
62:  │   └── 实际输出效果
96:  ├── 核心特性
107: ├── 进阶与协作
114: └── 许可证
```

## 核心特性

| 特性 | 说明 |
| :--- | :--- |
| **行号定位** | 提取每个标题的起始行号，便于跳转定位。 |
| **语法级精准解析** | 基于 `pulldown-cmark` 零拷贝引擎，忽略代码围栏与缩进块内部的 `#` 注释。 |
| **全语法风格兼容** | 原生兼容 ATX（`#`）与 Setext（下划线）两种标题语法。 |
| **多形态输出** | 原生支持 `markdown`（默认）、`tree` 与 `json` 三种模式。 |
| **纯净文本提取** | 自动剥离标题内的 Markdown 超链接、图片与格式标记，仅保留可读文本与行内代码。 |
| **极致体积与性能** | 原生 Rust 独立编译产物仅约 760KB，执行耗时低于 1 毫秒，零外部运行时依赖。 |

## 进阶与协作

- 系统架构与设计决策详见 [ARCHITECTURE.md](ARCHITECTURE.md)
- 本地开发与代码贡献准则详见 [CONTRIBUTING.md](CONTRIBUTING.md)
- 版本变更履历详见 [CHANGELOG.md](CHANGELOG.md)
- 安全策略与漏洞通报详见 [SECURITY.md](SECURITY.md)

## 许可证

本项目采用 [MIT License](LICENSE) 开源协议。
