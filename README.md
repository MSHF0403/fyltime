# fyltime

ファイルを更新時間で絞り込む、`ls`を拡張したシンプルなCLIツールです。

![Rust](https://img.shields.io/badge/Rust-CLI-orange)
[![License](https://img.shields.io/badge/license-GPLv2-blue)](LICENSE)
![Version](https://img.shields.io/badge/version-0.1.2-green)
[![Coverage Status](https://coveralls.io/repos/github/MSHF0403/fyltime/badge.svg?branch=main&v=2)](https://coveralls.io/github/MSHF0403/fyltime?branch=main)
![Build](https://github.com/MSHF0403/fyltime/actions/workflows/build.yaml/badge.svg)

## Overview

fyltimeは、ディレクトリ内のファイルを走査し、更新日時や拡張子などの条件でファイルを絞り込む軽量なCLIツールです。

最近更新されたファイルや、指定した日付より前後に更新されたファイルを簡単に抽出できます。ファイル整理や不要ファイルの発見を効率化することを目的としています。

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
      --completions         補完ファイルを生成
  -h, --help                ヘルプを表示
  -V, --version             バージョンを表示
