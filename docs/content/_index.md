---
title: "fyltime"
description: "Search files by modification time."
---

# fyltime

**fyltime** は、更新日時や拡張子を条件としてファイルを検索するコマンドラインツールです。

## Features

- Search files modified within a specified period (`--since`)
- Search files modified after a specified date (`--after`)
- Search files modified before a specified date (`--before`)
- Filter files by extension (`--ext`)
- Include hidden files (`--all`)

## Installation

Clone the repository and build with Cargo.

```bash
git clone https://github.com/MSHF0403/fyltime.git
cd fyltime
cargo build --release
```

Or install directly with Cargo.

```bash
cargo install --path .
```

## Usage

Display files modified within the last three days.

```bash
fyltime --since 3d
```

Display Rust source files.

```bash
fyltime --ext rs
```

Display files modified after January 1, 2026.

```bash
fyltime --after 2026-01-01
```

Display files modified before January 1, 2026.

```bash
fyltime --before 2026-01-01
```

Include hidden files.

```bash
fyltime --all
```

Options can also be combined.

```bash
fyltime --since 7d --ext rs --all
```

## Options

| Option | Description |
|--------|-------------|
| `--since <duration>` | Search files modified within the specified duration (e.g. `3d`, `12h`) |
| `--after <date>` | Search files modified after the specified date (`YYYY-MM-DD`) |
| `--before <date>` | Search files modified before the specified date (`YYYY-MM-DD`) |
| `--ext <extension>` | Search files with the specified extension |
| `-a`, `--all` | Include hidden files |
| `-h`, `--help` | Show help message |
| `-v`, `--version` | Show version information |

## License

MIT License.