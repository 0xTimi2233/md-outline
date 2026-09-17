# md-outline

A fast, AST-based Markdown outline CLI that extracts clean hierarchical heading trees with exact line numbers.

## Features

- **Accurate Line Numbers**: Exact 1-based source line mapping for every heading.
- **AST-Powered**: Zero false positives from `#` comments inside code blocks (fenced, tildes, or indented).
- **Setext & ATX Support**: Seamlessly parses both `# Heading` and underline `===` / `---` styles.
- **Clean Text**: Strips inline links, images, and formatting noise while preserving readable heading text.
- **Depth Control**: Flexible `-d, --depth` filtering.
- **Pipe / Stdin Friendly**: Supports standard Unix piping (`cat file.md | md-outline -`).
- **Tiny & Blazingly Fast**: Pure Rust binary (~800KB), sub-millisecond execution.

## Installation

### From Pre-built Binaries (GitHub Releases)
Download the latest binary for your platform from [Releases](https://github.com/0xTimi2233/md-outline/releases).

### From Source (Cargo)
```bash
cargo install --path .
```

## Usage

```bash
# Inspect markdown file outline
md-outline README.md

# Limit heading depth to H2
md-outline docs/spec.md -d 2

# Read from standard input
cat large-doc.md | md-outline -
```

### Example Output

```text
L5    md-outline
L9    ├── Features
L18   ├── Installation
L20   │   ├── From Pre-built Binaries (GitHub Releases)
L24   │   └── From Source (Cargo)
L29   ├── Usage
L39   └── Example Output
```

## License

MIT License.
