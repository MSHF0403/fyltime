//! `fyltime`は、更新日時や拡張子を条件として
//! ファイルを検索するコマンドラインツールです。
//!
//! 更新期間、指定日より前後、拡張子、隠しファイルの有無を
//! 条件としてファイルを絞り込めます。
//!
//! # Examples
//!
//! ```text
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

pub mod config;
mod gencomp;

use crate::config::{Config, parse_args};

/// `Cargo.toml`に設定されている現在のバージョンです。
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// コマンドライン引数を読み取り、ファイル検索を実行します。
pub fn run() {
    let args = parse_args();

    if args.completions {
        gencomp::generate(Path::new("completions"));
        return;
    }

    let config = args.to_config();

    match search_files(Path::new("."), &config) {
        Ok(paths) => {
            for path in paths {
                println!("{}", path.display());
            }
        }
        Err(error) => {
            eprintln!("ディレクトリを読み込めません: {}", error);
        }
    }
}

/// 指定したディレクトリから、設定に一致するファイルを検索します。
pub fn search_files(directory: &Path, config: &Config) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(directory)?;
    let now = SystemTime::now();
    let mut results = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();

        if !config.all && is_hidden(&path) {
            continue;
        }

        if !path.is_file() {
            continue;
        }

        if let Some(ref ext) = config.ext
            && path.extension().and_then(|s| s.to_str()) != Some(ext.as_str())
        {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(metadata) => metadata,
            Err(_) => continue,
        };

        let modified = match metadata.modified() {
            Ok(modified) => modified,
            Err(_) => continue,
        };

        if let Some(duration) = config.since {
            let elapsed = now.duration_since(modified).unwrap_or(Duration::ZERO);

            if elapsed > duration {
                continue;
            }
        }

        if let Some(after) = config.after
            && modified < after
        {
            continue;
        }

        if let Some(before) = config.before
            && modified > before
        {
            continue;
        }

        results.push(path);
    }

    results.sort();

    Ok(results)
}

/// `3d`や`12h`などの文字列を`Duration`へ変換します。
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

/// ファイル名が`.`から始まるかを確認します。
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
    fn normal_file_is_not_hidden() {
        assert!(!is_hidden(Path::new("README.md")));
    }
}
