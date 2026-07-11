use clap::Parser;
use std::time::{Duration, SystemTime};

use crate::{parse_date, parse_duration};

#[derive(Debug, Default)]
pub struct Config {
    pub since: Option<Duration>,
    pub after: Option<SystemTime>,
    pub before: Option<SystemTime>,
    pub ext: Option<String>,
    pub all: bool,
}

#[derive(Debug, Parser)]
#[command(
    name = "fyt",
    version,
    about = "更新日時や拡張子を条件としてファイルを検索します"
)]
pub struct Args {
    /// 指定した期間以内に更新されたファイルを表示します
    #[arg(long, value_name = "DURATION")]
    pub since: Option<String>,

    /// 指定した日付以降に更新されたファイルを表示します
    #[arg(long, value_name = "DATE")]
    pub after: Option<String>,

    /// 指定した日付以前に更新されたファイルを表示します
    #[arg(long, value_name = "DATE")]
    pub before: Option<String>,

    /// 指定した拡張子のファイルのみ表示します
    #[arg(long, value_name = "EXTENSION")]
    pub ext: Option<String>,

    /// 隠しファイルを含めて表示します
    #[arg(short = 'a', long)]
    pub all: bool,

    /// 補完ファイルを生成します
    #[arg(long, default_value_t = false)]
    pub completions: bool,
}

impl Args {
    pub fn to_config(&self) -> Config {
        Config {
            since: self.since.as_deref().and_then(parse_duration),
            after: self.after.as_deref().and_then(parse_date),
            before: self.before.as_deref().and_then(parse_date),
            ext: self.ext.clone(),
            all: self.all,
        }
    }
}

pub fn parse_args() -> Args {
    Args::parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_duration_days() {
        assert_eq!(
            parse_duration("3d"),
            Some(Duration::from_secs(3 * 24 * 60 * 60))
        );
    }

    #[test]
    fn parse_duration_hours() {
        assert_eq!(
            parse_duration("12h"),
            Some(Duration::from_secs(12 * 60 * 60))
        );
    }

    #[test]
    fn parse_duration_invalid() {
        assert_eq!(parse_duration("abc"), None);
    }

    #[test]
    fn parse_args_ext() {
        let args = Args::try_parse_from(["fyt", "--ext", "rs"]).unwrap();
        let config = args.to_config();

        assert_eq!(config.ext, Some("rs".to_string()));
    }

    #[test]
    fn parse_args_all() {
        let args = Args::try_parse_from(["fyt", "--all"]).unwrap();
        let config = args.to_config();

        assert!(config.all);
    }

    #[test]
    fn parse_args_since() {
        let args = Args::try_parse_from(["fyt", "--since", "3d"]).unwrap();
        let config = args.to_config();

        assert!(config.since.is_some());
    }
}
