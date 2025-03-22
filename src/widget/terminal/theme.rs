use iced::{Color, color};

pub enum TerminalTheme {
    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
}

pub struct TerminalPalette {
    pub cursor: Color,
    pub foreground: TerminalColor,
    pub background: TerminalColor,
}

impl TerminalPalette {
    pub const DRACULA: Self = Self {
        cursor: color!(248.0, 248.0, 242.0),
        foreground: TerminalColor {
            black: color!(38.0, 38.0, 38.0),
            red: color!(230.0, 71.0, 71.0),
            green: color!(66.0, 230.0, 108.0),
            yellow: color!(228.0, 243.0, 74.0),
            blue: color!(155.0, 107.0, 223.0),
            magenta: color!(227.0, 86.0, 167.0),
            cyan: color!(117.0, 215.0, 236.0),
            white: color!(248.0, 248.0, 242.0),
        },
        background: TerminalColor {
            black: color!(122.0, 122.0, 122.0),
            red: color!(255.0, 85.0, 85.0),
            green: color!(80.0, 250.0, 123.0),
            yellow: color!(241.0, 250.0, 140.0),
            blue: color!(189.0, 147.0, 249.0),
            magenta: color!(255.0, 121.0, 198.0),
            cyan: color!(139.0, 233.0, 253.0),
            white: color!(249.0, 249.0, 251.0),
        },
    };
}

impl TerminalTheme {
    pub fn get_theme(&self) -> TerminalPalette {
        match self {
            Self::Dracula => TerminalPalette::DRACULA,
            _ => TerminalPalette {
                cursor: color!(248.0, 248.0, 242.0),
                foreground: TerminalColor {
                    black: color!(38.0, 38.0, 38.0),
                    red: color!(230.0, 71.0, 71.0),
                    green: color!(66.0, 230.0, 108.0),
                    yellow: color!(228.0, 243.0, 74.0),
                    blue: color!(155.0, 107.0, 223.0),
                    magenta: color!(227.0, 86.0, 167.0),
                    cyan: color!(117.0, 215.0, 236.0),
                    white: color!(248.0, 248.0, 242.0),
                },
                background: TerminalColor {
                    black: color!(122.0, 122.0, 122.0),
                    red: color!(255.0, 85.0, 85.0),
                    green: color!(80.0, 250.0, 123.0),
                    yellow: color!(241.0, 250.0, 140.0),
                    blue: color!(189.0, 147.0, 249.0),
                    magenta: color!(255.0, 121.0, 198.0),
                    cyan: color!(139.0, 233.0, 253.0),
                    white: color!(249.0, 249.0, 251.0),
                },
            },
        }
    }
}

pub struct TerminalColor {
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub yellow: Color,
    pub blue: Color,
    pub magenta: Color,
    pub cyan: Color,
    pub white: Color,
}
