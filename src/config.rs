use std::time::{Duration, SystemTime};

use crate::{parse_date, parse_duration};

#[derive(Default)]
pub struct Config {
    pub since: Option<Duration>,
    pub after: Option<SystemTime>,
    pub before: Option<SystemTime>,
    pub ext: Option<String>,
    pub all: bool,
}

pub fn parse_args(args: &[String]) -> Config {
    let mut config = Config::default();
    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--since" => {
                if i + 1 < args.len() {
                    config.since = parse_duration(&args[i + 1]);
                    i += 1;
                }
            }
            "--after" => {
                if i + 1 < args.len() {
                    config.after = parse_date(&args[i + 1]);
                    i += 1;
                }
            }
            "--before" => {
                if i + 1 < args.len() {
                    config.before = parse_date(&args[i + 1]);
                    i += 1;
                }
            }
            "--ext" => {
                if i + 1 < args.len() {
                    config.ext = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "-a" | "--all" => {
                config.all = true;
            }
            _ => {}
        }

        i += 1;
    }

    config
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
        let args = vec![
            "fyltime".to_string(),
            "--ext".to_string(),
            "rs".to_string(),
        ];

        let config = parse_args(&args);

        assert_eq!(config.ext, Some("rs".to_string()));
    }

    #[test]
    fn parse_args_all() {
        let args = vec![
            "fyltime".to_string(),
            "--all".to_string(),
        ];

        let config = parse_args(&args);

        assert!(config.all);
    }

    #[test]
    fn parse_args_since() {
        let args = vec![
            "fyltime".to_string(),
            "--since".to_string(),
            "3d".to_string(),
        ];

        let config = parse_args(&args);

        assert!(config.since.is_some());
    }
}