# fyltime

ファイルを更新時間で絞り込むシンプルなlsを拡張したCLIツール

![Rust](https://img.shields.io/badge/Rust-CLI-orange)
[![License](https://img.shields.io/badge/license-GPLv2-blue)](LICENSE)
![Version](https://img.shields.io/badge/version-0.1.2-green)
[![Coverage Status](https://coveralls.io/repos/github/MSHF0403/fyltime/badge.svg?branch=main)](https://coveralls.io/github/MSHF0403/fyltime?branch=main)
![Build](https://github.com/MSHF0403/fyltime/actions/workflows/build.yaml/badge.svg)

## Overview
fyltimeは、ディレクトリ内のファイルを走査し、更新日時に基づいてファイルを絞り込む軽量なCLIツールです。  
複雑なコマンドを使用せず、直感的に古いファイルや最近更新されたファイルを抽出することを目的としています。  
ファイル整理や不要ファイルの発見を効率化するために設計されています。

## Usage
```text
Usage:
  fyt [OPTION]

OPTION:
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

「filter by time」をもとに、「fil」を変形した「fyl」と「time」を組み合わせた造語である。

バージョン履歴
v0.1.0：初期バージョン
```
