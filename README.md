# fyltime

ファイルを更新時間で絞り込むシンプルなlsを拡張したCLIツール

![Rust](https://img.shields.io/badge/Rust-CLI-orange)
![License](https://img.shields.io/badge/license-GPLv2-blue)
![Version](https://img.shields.io/badge/version-0.1.0-green)

## Overview
fyltimeは、ディレクトリ内のファイルを走査し、更新日時に基づいてファイルを絞り込む軽量なCLIツールです。  
複雑なコマンドを使用せず、直感的に古いファイルや最近更新されたファイルを抽出することを目的としています。  
ファイル整理や不要ファイルの発見を効率化するために設計されています。

## Usage
```text
Usage:
  fyltime [OPTION]

OPTION:
```text
  --since <期間>         指定した期間以内に更新されたファイルを表示（例: 3d, 12h）
  --after <日時>         指定した日時以降に更新されたファイルを表示（例: 2024-01-01）
  --before <日時>        指定した日時以前に更新されたファイルを表示
  --ext <拡張子>         指定した拡張子のファイルのみ表示（例: rs, txt）
  -a, --all              隠しファイルを含めて表示
  -h, --help             ヘルプを表示
  -v, --version          現在のバージョンを表示
```
## About
```text
開発者
MSHF0403

ライセンス
GNU General Public License v2.0

名前の由来
「file」と「time」を組み合わせた造語であり、ファイルを時間で扱うツールであることを表している。

### バージョン履歴
v0.1.0：初期バージョン
```
