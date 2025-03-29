use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub colors: Option<Colors>,
    pub general: Option<General>,
    pub keybinds: Option<Keybinds>,
    pub font: Option<Font>,
    pub bell: Option<Bell>,
}

#[derive(Deserialize, Clone)]
pub struct General {
    pub wallpaper: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Font {
    pub family: Option<String>,
    pub size: Option<f32>,
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum Command {
    Simple(String),
    Complex { program: String, args: Vec<String> },
}

#[derive(Deserialize, Clone)]
pub struct Bell {
    pub duration: Option<u64>,
    pub color: Option<String>,
    pub command: Option<Command>,
}

#[derive(Deserialize, Clone)]
pub struct Keybinds {
    pub new: Option<String>,
    pub close: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Colors {
    pub app: Option<App>,
    pub primary: Option<Primary>,
    pub normal: Option<TerminalColors>,
    pub bright: Option<TerminalColors>,
    pub dim: Option<TerminalColors>,
}

#[derive(Deserialize, Clone)]
pub struct App {
    pub background: Option<String>,
    pub active: Option<String>,
    pub inactive: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Primary {
    pub foreground: Option<String>,
    pub background: Option<String>,
    pub dim_foreground: Option<String>,
    pub bright_foreground: Option<String>,
}

#[derive(Deserialize, Clone)]
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
