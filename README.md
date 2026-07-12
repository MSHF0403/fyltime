# fyltime

ファイルを更新時間で絞り込むシンプルな`ls`拡張CLIツール

![Rust](https://img.shields.io/badge/Rust-CLI-orange)
[![License](https://img.shields.io/badge/license-GPLv2-blue)](LICENSE)
![Version](https://img.shields.io/badge/version-0.1.2-green)
[![Coverage Status](https://coveralls.io/repos/github/MSHF0403/fyltime/badge.svg?branch=main)](https://coveralls.io/github/MSHF0403/fyltime?branch=main)
[![Build](https://github.com/MSHF0403/fyltime/actions/workflows/build.yaml/badge.svg?branch=main)](https://github.com/MSHF0403/fyltime/actions/workflows/build.yaml?query=branch%3Amain)

## Overview

fyltimeは、ディレクトリ内のファイルを走査し、更新日時に基づいてファイルを絞り込む軽量なCLIツールです。  
複雑なコマンドを使用せず、直感的に古いファイルや最近更新されたファイルを抽出できます。  
ファイル整理や不要ファイルの発見を効率化することを目的として設計されています。

## Usage

```text
Usage:
  fyt [OPTIONS]

Options:
      --since <DURATION>    指定した期間以内に更新されたファイルを表示
      --after <DATE>        指定した日付以降に更新されたファイルを表示
      --before <DATE>       指定した日付以前に更新されたファイルを表示
      --ext <EXTENSION>     指定した拡張子のファイルのみ表示
  -a, --all                 隠しファイルを含めて表示
      --completions         シェル補完ファイルを生成
  -h, --help                ヘルプを表示
  -V, --version             バージョンを表示
```

## About

```text
開発者
MSHF0403

ライセンス
GNU General Public License v2.0

名前の由来
「filter by time」をもとに、「fil」を変形した「fyl」と
「time」を組み合わせた造語。

バージョン履歴
v0.1.0：初期バージョン
v0.1.1：シェル補完機能を追加
v0.1.2：CI/CD・Docker・テスト環境を改善
```
