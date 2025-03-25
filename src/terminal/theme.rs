use crate::terminal::settings::ThemeSettings;
use alacritty_terminal::vte::ansi::{self, NamedColor};
use iced::{Color, widget::container};
use std::collections::HashMap;

pub(crate) trait TerminalStyle {
    fn container_style(&self) -> container::Style;
}

#[derive(Debug, Clone)]
pub struct ColorPalette {
    pub foreground: String,
    pub background: String,
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
    pub bright_black: String,
    pub bright_red: String,
    pub bright_green: String,
    pub bright_yellow: String,
    pub bright_blue: String,
    pub bright_magenta: String,
    pub bright_cyan: String,
    pub bright_white: String,
    pub bright_foreground: Option<String>,
    pub dim_foreground: String,
    pub dim_black: String,
    pub dim_red: String,
    pub dim_green: String,
    pub dim_yellow: String,
    pub dim_blue: String,
    pub dim_magenta: String,
    pub dim_cyan: String,
    pub dim_white: String,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            foreground: String::from("#cdd6f4"),
            background: String::from("#1e1e2e"),
            black: String::from("#45475a"),
            red: String::from("#f38ba8"),
            green: String::from("#a6e3a1"),
            yellow: String::from("#f9e2af"),
            blue: String::from("#89b4fa"),
            magenta: String::from("#f5c2e7"),
            cyan: String::from("#94e2d5"),
            white: String::from("#bac2de"),
            bright_black: String::from("#585b70"),
            bright_red: String::from("#f38ba8"),
            bright_green: String::from("#a6e3a1"),
            bright_yellow: String::from("#f9e2af"),
            bright_blue: String::from("#89b4fa"),
            bright_magenta: String::from("#f5c2e7"),
            bright_cyan: String::from("#94e2d5"),
            bright_white: String::from("#a6adc8"),
            bright_foreground: None,
            dim_foreground: String::from("#cdd6f4"),
            dim_black: String::from("#45475a"),
            dim_red: String::from("#f38ba8"),
            dim_green: String::from("#a6e3a1"),
            dim_yellow: String::from("#f9e2af"),
            dim_blue: String::from("#89b4fa"),
            dim_magenta: String::from("#f5c2e7"),
            dim_cyan: String::from("#94e2d5"),
            dim_white: String::from("#bac2de"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    palette: Box<ColorPalette>,
    ansi256_colors: HashMap<u8, Color>,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            palette: Box::<ColorPalette>::default(),
            ansi256_colors: build_ansi256_colors(),
        }
    }
}

impl Theme {
    pub fn new(settings: ThemeSettings) -> Self {
        Self {
            palette: settings.color_pallete,
            ansi256_colors: build_ansi256_colors(),
        }
    }

    pub fn get_color(&self, c: ansi::Color) -> Color {
        match c {
            ansi::Color::Spec(rgb) => Color::from_rgb8(rgb.r, rgb.g, rgb.b),
            ansi::Color::Indexed(index) => {
                if index <= 15 {
                    let color = match index {
                        // Normal terminal colors
                        0 => &self.palette.black,
                        1 => &self.palette.red,
                        2 => &self.palette.green,
                        3 => &self.palette.yellow,
                        4 => &self.palette.blue,
                        5 => &self.palette.magenta,
                        6 => &self.palette.cyan,
                        7 => &self.palette.white,
                        // Bright terminal colors
                        8 => &self.palette.bright_black,
                        9 => &self.palette.bright_red,
                        10 => &self.palette.bright_green,
                        11 => &self.palette.bright_yellow,
                        12 => &self.palette.bright_blue,
                        13 => &self.palette.bright_magenta,
                        14 => &self.palette.bright_cyan,
                        15 => &self.palette.bright_white,
                        _ => &self.palette.background,
                    };

                    return hex_to_color(color)
                        .unwrap_or_else(|_| panic!("invalid color {}", color));
                }

                // Other colors
                match self.ansi256_colors.get(&index) {
                    Some(color) => *color,
                    None => Color::from_rgb8(0, 0, 0),
                }
            }
            ansi::Color::Named(c) => {
                let color = match c {
                    NamedColor::Foreground => &self.palette.foreground,
                    NamedColor::Background => &self.palette.background,
                    // Normal terminal colors
                    NamedColor::Black => &self.palette.black,
                    NamedColor::Red => &self.palette.red,
                    NamedColor::Green => &self.palette.green,
                    NamedColor::Yellow => &self.palette.yellow,
                    NamedColor::Blue => &self.palette.blue,
                    NamedColor::Magenta => &self.palette.magenta,
                    NamedColor::Cyan => &self.palette.cyan,
                    NamedColor::White => &self.palette.white,
                    // Bright terminal colors
                    NamedColor::BrightBlack => &self.palette.bright_black,
                    NamedColor::BrightRed => &self.palette.bright_red,
                    NamedColor::BrightGreen => &self.palette.bright_green,
                    NamedColor::BrightYellow => &self.palette.bright_yellow,
                    NamedColor::BrightBlue => &self.palette.bright_blue,
                    NamedColor::BrightMagenta => &self.palette.bright_magenta,
                    NamedColor::BrightCyan => &self.palette.bright_cyan,
                    NamedColor::BrightWhite => &self.palette.bright_white,
                    NamedColor::BrightForeground => match &self.palette.bright_foreground {
                        Some(color) => color,
                        None => &self.palette.foreground,
                    },
                    // Dim terminal colors
                    NamedColor::DimForeground => &self.palette.dim_foreground,
                    NamedColor::DimBlack => &self.palette.dim_black,
                    NamedColor::DimRed => &self.palette.dim_red,
                    NamedColor::DimGreen => &self.palette.dim_green,
                    NamedColor::DimYellow => &self.palette.dim_yellow,
                    NamedColor::DimBlue => &self.palette.dim_blue,
                    NamedColor::DimMagenta => &self.palette.dim_magenta,
                    NamedColor::DimCyan => &self.palette.dim_cyan,
                    NamedColor::DimWhite => &self.palette.dim_white,
                    _ => &self.palette.background,
                };

                hex_to_color(color).unwrap_or_else(|_| panic!("invalid color {}", color))
            }
        }
    }
}

fn build_ansi256_colors() -> HashMap<u8, Color> {
    let mut ansi256_colors = HashMap::new();

    for r in 0..6 {
        for g in 0..6 {
            for b in 0..6 {
                // Reserve the first 16 colors for config.
                let index = 16 + r * 36 + g * 6 + b;
                let color = Color::from_rgb8(
                    if r == 0 { 0 } else { r * 40 + 55 },
                    if g == 0 { 0 } else { g * 40 + 55 },
                    if b == 0 { 0 } else { b * 40 + 55 },
                );
                ansi256_colors.insert(index, color);
            }
        }
    }

    let index: u8 = 232;
    for i in 0..24 {
        let value = i * 10 + 8;
        ansi256_colors.insert(index + i, Color::from_rgb8(value, value, value));
    }

    ansi256_colors
}

fn hex_to_color(hex: &str) -> anyhow::Result<Color> {
    if hex.len() != 7 {
        return Err(anyhow::format_err!("input string is in non valid format"));
    }

    let r = u8::from_str_radix(&hex[1..3], 16)?;
    let g = u8::from_str_radix(&hex[3..5], 16)?;
    let b = u8::from_str_radix(&hex[5..7], 16)?;

    Ok(Color::from_rgb8(r, g, b))
}

impl TerminalStyle for Theme {
    fn container_style(&self) -> container::Style {
        container::Style {
            background: Some(
                hex_to_color(&self.palette.background)
                    .unwrap_or_else(|_| {
                        panic!("invalid background color {}", self.palette.background)
                    })
                    .into(),
            ),
            ..container::Style::default()
        }
    }
}
