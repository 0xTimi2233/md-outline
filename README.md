# md-outline

基于 CommonMark 抽象语法树（AST）的高性能 Markdown 大纲提取工具，毫秒级提取具备精确绝对行号与层级连线的结构树。

## 核心特性

- **绝对行号定位**：精准提取每个标题在源文件中的真实起始行号，便于快速跳转与定位。
- **抽象语法树解析**：基于 `pulldown-cmark` 流式零拷贝解析引擎，彻底杜绝围栏代码块、波浪线代码块以及缩进代码块内部注释被误判为标题的问题。
- **全语法风格兼容**：原生支持 `#` 引导的 ATX 标题以及下划线形式的 Setext 标题。
- **纯净文本提取**：自动剥离标题内的 Markdown 超链接、图片与格式标记，仅保留可读文本与行内代码。
- **深度灵活过滤**：支持通过 `-d, --depth` 控制提取的最大标题层级。
- **标准管道输入**：自动感知标准输入流，无缝配合 Unix 管道命令。
- **极致体积与性能**：原生 Rust 独立编译产物仅约 760KB，执行耗时低于 1 毫秒，零外部运行时依赖。

## 安装指南

### 方式一：一键自动安装（推荐）

#### macOS 与 Linux

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/0xTimi2233/md-outline/releases/latest/download/md-outline-installer.sh | sh
```

#### Windows (PowerShell)

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/0xTimi2233/md-outline/releases/latest/download/md-outline-installer.ps1 | iex"
```

### 方式二：通过 cargo-binstall 极速安装

无需本地编译，直接拉取预编译二进制：

```bash
cargo binstall md-outline
```

### 方式三：从 GitHub Releases 下载预编译归档包

前往 [Releases 页面](https://github.com/0xTimi2233/md-outline/releases) 下载适配当前平台的归档包：

- macOS Apple Silicon：`md-outline-aarch64-apple-darwin.tar.xz`
- macOS Intel：`md-outline-x86_64-apple-darwin.tar.xz`
- Linux x86_64：`md-outline-x86_64-unknown-linux-gnu.tar.xz`
- Linux ARM64：`md-outline-aarch64-unknown-linux-gnu.tar.xz`
- Windows x64：`md-outline-x86_64-pc-windows-msvc.zip`

解压后将可执行文件放置于系统 PATH 目录（如 `~/.local/bin` 或 `~/.cargo/bin`）。现代 macOS 与 Linux 系统自带的 `tar -xf` 命令原生支持直接解压 `.tar.xz`。

### 方式四：通过源码编译安装

```bash
cargo install --path .
```

## 使用指南

```bash
# 提取指定 Markdown 文件的大纲
md-outline README.md

# 仅查看二级及以上的大纲层级
md-outline docs/spec.md -d 2

# 通过 Unix 管道读取标准输入
cat large-doc.md | md-outline
```

### 实际输出效果

```text
L1    md-outline
L5    ├── 核心特性
L15   ├── 安装指南
L17   │   ├── 方式一：一键自动安装（推荐）
L27   │   ├── 方式二：通过 cargo-binstall 极速安装
L33   │   ├── 方式三：从 GitHub Releases 下载预编译归档包
L43   │   └── 方式四：通过源码编译安装
L49   ├── 使用指南
L59   └── 实际输出效果
```

## 贡献与发布

项目采用云端统一构建与跨平台自动化发布，本地开发遵循双层垂直测试驱动开发规范。详细流程请阅读 [CONTRIBUTING.md](CONTRIBUTING.md)。

```bash
# 执行本地全量测试与代码门禁
just ci

# 从云端 Release 更新本地安装态
just update
```

## 开源协议

本项目采用 [MIT License](LICENSE) 开源协议。
