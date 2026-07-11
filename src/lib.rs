//! `fyltime`は、更新日時や拡張子を条件として
//! ファイルやディレクトリを検索するコマンドラインツールです。
//!
//! 更新期間、指定日より前後、拡張子、隠しファイルの有無を
//! 条件として項目を絞り込めます。
//!
//! # Examples
//!
//! ```text
//! fyt
//! fyt --since 3d
//! fyt --after 2026-01-01
//! fyt --before 2026-12-31 --ext rs
//! fyt --all
//! ```

use chrono::NaiveDate;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};
use terminal_size::{Width, terminal_size};

pub mod config;
mod gencomp;

use crate::config::{Config, parse_args};

/// `Cargo.toml`に設定されている現在のバージョンです。
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// コマンドライン引数を読み取り、項目の検索を実行します。
pub fn run() {
    let args = parse_args();

    if args.completions {
        gencomp::generate(Path::new("completions"));
        return;
    }

    let config = args.to_config();

    match search_files(Path::new("."), &config) {
        Ok(paths) => {
            print_columns(&paths);
        }
        Err(error) => {
            eprintln!("ディレクトリを読み込めません: {error}");
        }
    }
}

/// 指定したディレクトリから、設定に一致する項目を検索します。
///
/// オプションなしの場合は、ファイルとディレクトリの両方を返します。
/// `--ext`が指定された場合は、該当する拡張子のファイルだけを返します。
pub fn search_files(directory: &Path, config: &Config) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(directory)?;
    let now = SystemTime::now();
    let mut results = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();

        // --allが指定されていない場合は隠し項目を除外する
        if !config.all && is_hidden(&path) {
            continue;
        }

        // --extが指定された場合は、該当する拡張子のファイルだけを対象にする
        if let Some(ref extension) = config.ext {
            if !path.is_file()
                || path.extension().and_then(|value| value.to_str()) != Some(extension.as_str())
            {
                continue;
            }
        }

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };

        let modified = match metadata.modified() {
            Ok(modified) => modified,
            Err(_) => continue,
        };

        // 現在から指定期間以内に更新された項目だけを対象にする
        if let Some(duration) = config.since {
            let elapsed = now.duration_since(modified).unwrap_or(Duration::ZERO);

            if elapsed > duration {
                continue;
            }
        }

        // 指定日時以降に更新された項目だけを対象にする
        if let Some(after) = config.after
            && modified < after
        {
            continue;
        }

        // 指定日時以前に更新された項目だけを対象にする
        if let Some(before) = config.before
            && modified > before
        {
            continue;
        }

        results.push(path);
    }

    // ファイル名を基準に、大文字と小文字を区別せず並べる
    results.sort_by(|a, b| {
        let a_name = file_name_string(a).to_lowercase();
        let b_name = file_name_string(b).to_lowercase();

        a_name
            .cmp(&b_name)
            .then_with(|| file_name_string(a).cmp(&file_name_string(b)))
    });

    Ok(results)
}

/// 検索結果をターミナル幅に合わせて、lsのように列方向へ並べて表示します。
fn print_columns(paths: &[PathBuf]) {
    if paths.is_empty() {
        return;
    }

    let names: Vec<String> = paths.iter().map(|path| file_name_string(path)).collect();

    let column_width = names
        .iter()
        .map(|name| name.chars().count())
        .max()
        .unwrap_or(0)
        + 2;

    let terminal_width = terminal_size()
        .map(|(Width(width), _)| usize::from(width))
        .unwrap_or(80);

    let columns = (terminal_width / column_width).max(1);
    let rows = names.len().div_ceil(columns);

    for row in 0..rows {
        for column in 0..columns {
            let index = column * rows + row;

            if index >= names.len() {
                continue;
            }

            let name = &names[index];

            let has_next_item =
                (column + 1..columns).any(|next_column| next_column * rows + row < names.len());

            if has_next_item {
                let padding = column_width.saturating_sub(name.chars().count());
                print!("{name}{}", " ".repeat(padding));
            } else {
                print!("{name}");
            }
        }

        println!();
    }
}

/// パスからファイル名またはディレクトリ名を文字列として取得します。
fn file_name_string(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| path.display().to_string())
}

/// `3d`や`12h`などの文字列を`Duration`へ変換します。
///
/// 対応している単位は以下です。
///
/// - `s`: 秒
/// - `m`: 分
/// - `h`: 時間
/// - `d`: 日
pub fn parse_duration(value: &str) -> Option<Duration> {
    if value.len() < 2 {
        return None;
    }

    let (number_part, unit) = value.split_at(value.len() - 1);
    let number: u64 = number_part.parse().ok()?;

    match unit {
        "s" => Some(Duration::from_secs(number)),
        "m" => Some(Duration::from_secs(number * 60)),
        "h" => Some(Duration::from_secs(number * 60 * 60)),
        "d" => Some(Duration::from_secs(number * 60 * 60 * 24)),
        _ => None,
    }
}

/// `YYYY-MM-DD`形式の日付を`SystemTime`へ変換します。
pub fn parse_date(value: &str) -> Option<SystemTime> {
    let date = NaiveDate::parse_from_str(value, "%Y-%m-%d").ok()?;
    let datetime = date.and_hms_opt(0, 0, 0)?.and_utc();

    Some(datetime.into())
}

/// ファイル名またはディレクトリ名が`.`から始まるかを確認します。
pub fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with('.'))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_duration_seconds() {
        assert_eq!(parse_duration("30s"), Some(Duration::from_secs(30)));
    }

    #[test]
    fn parse_duration_minutes() {
        assert_eq!(parse_duration("5m"), Some(Duration::from_secs(300)));
    }

    #[test]
    fn parse_duration_hours() {
        assert_eq!(parse_duration("2h"), Some(Duration::from_secs(2 * 60 * 60)));
    }

    #[test]
    fn parse_duration_days() {
        assert_eq!(
            parse_duration("3d"),
            Some(Duration::from_secs(3 * 24 * 60 * 60))
        );
    }

    #[test]
    fn parse_duration_invalid() {
        assert_eq!(parse_duration("3x"), None);
        assert_eq!(parse_duration("d"), None);
        assert_eq!(parse_duration("abc"), None);
    }

    #[test]
    fn parse_date_valid() {
        assert!(parse_date("2026-07-12").is_some());
    }

    #[test]
    fn parse_date_invalid() {
        assert!(parse_date("2026-99-99").is_none());
        assert!(parse_date("invalid").is_none());
    }

    #[test]
    fn hidden_file_is_detected() {
        assert!(is_hidden(Path::new(".gitignore")));
    }

    #[test]
    fn hidden_directory_is_detected() {
        assert!(is_hidden(Path::new(".git")));
    }

    #[test]
    fn normal_file_is_not_hidden() {
        assert!(!is_hidden(Path::new("README.md")));
    }

    #[test]
    fn normal_directory_is_not_hidden() {
        assert!(!is_hidden(Path::new("src")));
    }

    #[test]
    fn file_name_is_extracted() {
        assert_eq!(
            file_name_string(Path::new("./README.md")),
            "README.md".to_string()
        );
    }
}
