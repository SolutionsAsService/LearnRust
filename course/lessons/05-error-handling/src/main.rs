//! Lesson 05: errors as values: custom error types, `Display` and `?`.

use std::fmt;

#[derive(Debug, PartialEq)]
struct Config {
    name: String,
    port: u16,
    debug: bool,
}

#[derive(Debug, PartialEq)]
enum ConfigError {
    MissingEquals { line: usize },
    UnknownKey { line: usize, key: String },
    BadPort { line: usize, value: String },
    BadBool { line: usize, value: String },
    Missing(&'static str),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingEquals { line } => write!(f, "line {line}: expected key=value"),
            ConfigError::UnknownKey { line, key } => write!(f, "line {line}: unknown key `{key}`"),
            ConfigError::BadPort { line, value } => write!(f, "line {line}: `{value}` is not a port (0-65535)"),
            ConfigError::BadBool { line, value } => write!(f, "line {line}: `{value}` is not true or false"),
            ConfigError::Missing(key) => write!(f, "missing required key `{key}`"),
        }
    }
}

impl std::error::Error for ConfigError {}

fn parse_config(text: &str) -> Result<Config, ConfigError> {
    let (mut name, mut port, mut debug) = (None, None, false);
    for (i, raw) in text.lines().enumerate() {
        let line = i + 1;
        let raw = raw.trim();
        if raw.is_empty() || raw.starts_with('#') {
            continue;
        }
        let (key, value) = raw.split_once('=').ok_or(ConfigError::MissingEquals { line })?;
        let (key, value) = (key.trim(), value.trim());
        match key {
            "name" => name = Some(value.to_string()),
            "port" => port = Some(value.parse().map_err(|_| ConfigError::BadPort { line, value: value.into() })?),
            "debug" => debug = value.parse().map_err(|_| ConfigError::BadBool { line, value: value.into() })?,
            other => return Err(ConfigError::UnknownKey { line, key: other.into() }),
        }
    }
    Ok(Config { name: name.ok_or(ConfigError::Missing("name"))?, port: port.ok_or(ConfigError::Missing("port"))?, debug })
}

fn main() {
    let good = "# server\nname = learnrust\nport = 8080\ndebug = true\n";
    let bad = "name = learnrust\nport = 99999\n";
    for text in [good, bad] {
        match parse_config(text) {
            Ok(cfg) => println!("ok: {cfg:?}"),
            Err(e) => println!("error: {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_good_file() {
        let c = parse_config("name=a\nport=80").unwrap();
        assert_eq!(c, Config { name: "a".into(), port: 80, debug: false });
    }

    #[test]
    fn reports_the_line_of_a_bad_port() {
        assert_eq!(parse_config("name=a\nport=70000").unwrap_err(), ConfigError::BadPort { line: 2, value: "70000".into() });
    }

    #[test]
    fn needs_a_name() {
        assert_eq!(parse_config("port=1").unwrap_err().to_string(), "missing required key `name`");
    }
}
