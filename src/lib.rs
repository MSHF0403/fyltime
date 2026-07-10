use chrono::NaiveDate;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};

pub mod config;

use crate::config::parse_args;

pub const VERSION: &str = "0.1.0";

pub fn run() {
    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "-h" || a == "--help") {
        print_help();
        return;
    }

    if args.iter().any(|a| a == "-v" || a == "--version") {
        println!("fyltime {}", VERSION);
        return;
    }

    let config = parse_args(&args);

    let entries = match fs::read_dir(".") {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("ディレクトリを読み込めません: {}", e);
            return;
        }
    };

    let now = SystemTime::now();

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();

        if !config.all && is_hidden(&path) {
            continue;
        }

        if !path.is_file() {
            continue;
        }

        if let Some(ref ext) = config.ext {
            if path.extension().and_then(|s| s.to_str()) != Some(ext) {
                continue;
            }
        }

        let metadata = match fs::metadata(&path) {
            Ok(m) => m,
            Err(_) => continue,
        };

        let modified = match metadata.modified() {
            Ok(t) => t,
            Err(_) => continue,
        };

        if let Some(duration) = config.since {
            let elapsed = now.duration_since(modified).unwrap_or(Duration::ZERO);

            if elapsed > duration {
                continue;
            }
        }

        if let Some(after) = config.after {
            if modified < after {
                continue;
            }
        }

        if let Some(before) = config.before {
            if modified > before {
                continue;
            }
        }

        println!("{}", path.display());
    }
}

pub fn parse_duration(s: &str) -> Option<Duration> {
    if s.len() < 2 {
        return None;
    }

    let number_part = &s[..s.len() - 1];
    let unit = &s[s.len() - 1..];

    let value: u64 = number_part.parse().ok()?;

    match unit {
        "s" => Some(Duration::from_secs(value)),
        "m" => Some(Duration::from_secs(value * 60)),
        "h" => Some(Duration::from_secs(value * 60 * 60)),
        "d" => Some(Duration::from_secs(value * 60 * 60 * 24)),
        _ => None,
    }
}

pub fn parse_date(s: &str) -> Option<SystemTime> {
    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d").ok()?;
    let datetime = date.and_hms_opt(0, 0, 0)?.and_utc();

    Some(datetime.into())
}

pub fn is_hidden(path: &PathBuf) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

pub fn print_help() {
    println!(
        "\
Usage:
  fyltime [OPTION]

OPTION:
  --since <期間>         指定した期間以内に更新されたファイルを表示（例: 3d, 12h）
  --after <日時>         指定した日時以降に更新されたファイルを表示（例: 2024-01-01）
  --before <日時>        指定した日時以前に更新されたファイルを表示（例: 2024-01-01）
  --ext <拡張子>         指定した拡張子のファイルのみ表示（例: rs, txt）
  -a, --all              隠しファイルを含めて表示
  -h, --help             ヘルプを表示
  -v, --version          現在のバージョンを表示
"
    );
}