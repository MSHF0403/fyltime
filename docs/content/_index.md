---
title: "fyltime"
description: "更新日時を条件にファイルを検索する。"
---

**fyltime** は、更新日時や拡張子を条件としてファイルを検索するコマンドラインツールです。

## Features

- 指定した期間内に更新されたファイルを検索 (`--since`)
- 指定した日付以降に更新されたファイルを検索 (`--after`)
- 指定した日付以前に更新されたファイルを検索 (`--before`)
- 拡張子で絞り込み (`--ext`)
- 隠しファイルを含めて検索 (`--all`)

## Installation

リポジトリをクローンして、Cargoでビルドします。

```bash
git clone https://github.com/MSHF0403/fyt.git
cd fyt
cargo build --release
```

または、Cargoから直接インストールできます。

```bash
cargo install --path .
```

## Usage

直近3日以内に更新されたファイルを表示します。

```bash
fyt --since 3d
```

Rustソースファイルを表示します。

```bash
fyt --ext rs
```

2026年1月1日以降に更新されたファイルを表示します。

```bash
fyt --after 2026-01-01
```

2026年1月1日以前に更新されたファイルを表示します。

```bash
fyt --before 2026-01-01
```

隠しファイルを含めて表示します。

```bash
fyt --all
```

オプションは組み合わせて使用できます。

```bash
fyt --since 7d --ext rs --all
```

## Options

| Option | Description |
|--------|-------------|
| `--since <duration>` | 指定した期間内に更新されたファイルを検索します（例: `3d`, `12h`） |
| `--after <date>` | 指定した日付以降に更新されたファイルを検索します（`YYYY-MM-DD`） |
| `--before <date>` | 指定した日付以前に更新されたファイルを検索します（`YYYY-MM-DD`） |
| `--ext <extension>` | 指定した拡張子のファイルを検索します |
| `-a`, `--all` | 隠しファイルを検索対象に含めます |
| `-h`, `--help` | ヘルプを表示します |
| `-v`, `--version` | バージョン情報を表示します |

## License

MITライセンスです。