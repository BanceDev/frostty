use std::fs;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub colors: Option<Colors>,
}

#[derive(Deserialize)]
pub struct Colors {
    pub app: Option<App>,
    pub primary: Option<Primary>,
    pub normal: Option<TerminalColors>,
    pub bright: Option<TerminalColors>,
}

#[derive(Deserialize)]
pub struct App {
    pub background: Option<String>,
    pub active: Option<String>,
    pub inactive: Option<String>,
}

#[derive(Deserialize)]
pub struct Primary {
    pub foreground: Option<String>,
    pub background: Option<String>,
    pub dim_foreground: Option<String>,
    pub bright_foreground: Option<String>,
}

#[derive(Deserialize)]
pub struct TerminalColors {
    pub black: Option<String>,
    pub red: Option<String>,
    pub green: Option<String>,
    pub yellow: Option<String>,
    pub blue: Option<String>,
    pub magenta: Option<String>,
    pub cyan: Option<String>,
    pub white: Option<String>,
}

impl Config {
    pub fn new() -> Option<Self> {
        let filename = format!("{}/.config/frostty/frostty.toml", env!("HOME"));

        let contents = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("no config file found");
                return None;
            }
        };
        let config: Config = toml::from_str(&contents).expect("failed to parse config");
        Some(config)
    }
}
