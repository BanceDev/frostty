use crate::config::Config;
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
        let mut foreground = String::from("#d8d8d8");
        let mut background = String::from("#181818");
        let mut black = String::from("#181818");
        let mut red = String::from("#ac4242");
        let mut green = String::from("#90a959");
        let mut yellow = String::from("#f4bf75");
        let mut blue = String::from("#6a9fb5");
        let mut magenta = String::from("#aa759f");
        let mut cyan = String::from("#75b5aa");
        let mut white = String::from("#d8d8d8");
        let mut bright_black = String::from("#6b6b6b");
        let mut bright_red = String::from("#c55555");
        let mut bright_green = String::from("#aac474");
        let mut bright_yellow = String::from("#feca88");
        let mut bright_blue = String::from("#82b8c8");
        let mut bright_magenta = String::from("#c28cb8");
        let mut bright_cyan = String::from("#93d3c3");
        let mut bright_white = String::from("#f8f8f8");
        let mut bright_foreground = None;
        let mut dim_foreground = String::from("#828482");
        let mut dim_black = String::from("#0f0f0f");
        let mut dim_red = String::from("#712b2b");
        let mut dim_green = String::from("#5f6f3a");
        let mut dim_yellow = String::from("#a17e4d");
        let mut dim_blue = String::from("#456877");
        let mut dim_magenta = String::from("#704d68");
        let mut dim_cyan = String::from("#4d7770");
        let mut dim_white = String::from("#8e8e8e");

        if let Some(primary) = Config::new()
            .and_then(|config| config.colors)
            .and_then(|colors| colors.primary)
        {
            if let Some(fg) = primary.foreground {
                foreground = fg;
            }
            if let Some(bg) = primary.background {
                background = bg;
            }
            if let Some(dfg) = primary.dim_foreground {
                dim_foreground = dfg;
            }
            if let Some(bfg) = primary.bright_foreground {
                bright_foreground = Some(bfg);
            }
        }

        if let Some(normal) = Config::new()
            .and_then(|config| config.colors)
            .and_then(|colors| colors.normal)
        {
            if let Some(b) = normal.black {
                black = b;
            }
            if let Some(r) = normal.red {
                red = r;
            }
            if let Some(g) = normal.green {
                green = g;
            }
            if let Some(y) = normal.yellow {
                yellow = y;
            }
            if let Some(u) = normal.blue {
                blue = u;
            }
            if let Some(m) = normal.magenta {
                magenta = m;
            }
            if let Some(c) = normal.cyan {
                cyan = c;
            }
            if let Some(w) = normal.white {
                white = w;
            }
        }

        if let Some(bright) = Config::new()
            .and_then(|config| config.colors)
            .and_then(|colors| colors.bright)
        {
            if let Some(b) = bright.black {
                bright_black = b;
            }
            if let Some(r) = bright.red {
                bright_red = r;
            }
            if let Some(g) = bright.green {
                bright_green = g;
            }
            if let Some(y) = bright.yellow {
                bright_yellow = y;
            }
            if let Some(u) = bright.blue {
                bright_blue = u;
            }
            if let Some(m) = bright.magenta {
                bright_magenta = m;
            }
            if let Some(c) = bright.cyan {
                bright_cyan = c;
            }
            if let Some(w) = bright.white {
                bright_white = w;
            }
        }

        if let Some(dim) = Config::new()
            .and_then(|config| config.colors)
            .and_then(|colors| colors.dim)
        {
            if let Some(b) = dim.black {
                dim_black = b;
            }
            if let Some(r) = dim.red {
                dim_red = r;
            }
            if let Some(g) = dim.green {
                dim_green = g;
            }
            if let Some(y) = dim.yellow {
                dim_yellow = y;
            }
            if let Some(u) = dim.blue {
                dim_blue = u;
            }
            if let Some(m) = dim.magenta {
                dim_magenta = m;
            }
            if let Some(c) = dim.cyan {
                dim_cyan = c;
            }
            if let Some(w) = dim.white {
                dim_white = w;
            }
        }

        Self {
            foreground,
            background,
            black,
            red,
            green,
            yellow,
            blue,
            magenta,
            cyan,
            white,
            bright_black,
            bright_red,
            bright_green,
            bright_yellow,
            bright_blue,
            bright_magenta,
            bright_cyan,
            bright_white,
            bright_foreground,
            dim_foreground,
            dim_black,
            dim_red,
            dim_green,
            dim_yellow,
            dim_blue,
            dim_magenta,
            dim_cyan,
            dim_white,
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
    if hex.len() != 7 && hex.len() != 9 {
        return Err(anyhow::format_err!("input string is in non valid format"));
    }

    let r = u8::from_str_radix(&hex[1..3], 16)?;
    let g = u8::from_str_radix(&hex[3..5], 16)?;
    let b = u8::from_str_radix(&hex[5..7], 16)?;
    if hex.len() == 9 {
        let a = u8::from_str_radix(&hex[7..9], 16)? as f32 / 255.0;
        Ok(Color::from_rgba8(r, g, b, a))
    } else {
        Ok(Color::from_rgb8(r, g, b))
    }
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
