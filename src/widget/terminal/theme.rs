use iced::Color;

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
    cursor: Color,
    foreground: TerminalColor,
    background: TerminalColor,
}

impl TerminalTheme {
    fn get_theme(&self) -> TerminalPalette {
        match self {
            Self::Dracula => TerminalPalette {
                cursor: Color::from_rgb(248.0, 248.0, 242.0),
                foreground: TerminalColor {
                    black: Color::from_rgb(38.0, 38.0, 38.0),
                    red: Color::from_rgb(230.0, 71.0, 71.0),
                    green: Color::from_rgb(66.0, 230.0, 108.0),
                    yellow: Color::from_rgb(228.0, 243.0, 74.0),
                    blue: Color::from_rgb(155.0, 107.0, 223.0),
                    magenta: Color::from_rgb(227.0, 86.0, 167.0),
                    cyan: Color::from_rgb(117.0, 215.0, 236.0),
                    white: Color::from_rgb(248.0, 248.0, 242.0),
                },
                background: TerminalColor {
                    black: Color::from_rgb(122.0, 122.0, 122.0),
                    red: Color::from_rgb(255.0, 85.0, 85.0),
                    green: Color::from_rgb(80.0, 250.0, 123.0),
                    yellow: Color::from_rgb(241.0, 250.0, 140.0),
                    blue: Color::from_rgb(189.0, 147.0, 249.0),
                    magenta: Color::from_rgb(255.0, 121.0, 198.0),
                    cyan: Color::from_rgb(139.0, 233.0, 253.0),
                    white: Color::from_rgb(249.0, 249.0, 251.0),
                },
            },
            _ => TerminalPalette {
                cursor: Color::from_rgb(248.0, 248.0, 242.0),
                foreground: TerminalColor {
                    black: Color::from_rgb(38.0, 38.0, 38.0),
                    red: Color::from_rgb(230.0, 71.0, 71.0),
                    green: Color::from_rgb(66.0, 230.0, 108.0),
                    yellow: Color::from_rgb(228.0, 243.0, 74.0),
                    blue: Color::from_rgb(155.0, 107.0, 223.0),
                    magenta: Color::from_rgb(227.0, 86.0, 167.0),
                    cyan: Color::from_rgb(117.0, 215.0, 236.0),
                    white: Color::from_rgb(248.0, 248.0, 242.0),
                },
                background: TerminalColor {
                    black: Color::from_rgb(122.0, 122.0, 122.0),
                    red: Color::from_rgb(255.0, 85.0, 85.0),
                    green: Color::from_rgb(80.0, 250.0, 123.0),
                    yellow: Color::from_rgb(241.0, 250.0, 140.0),
                    blue: Color::from_rgb(189.0, 147.0, 249.0),
                    magenta: Color::from_rgb(255.0, 121.0, 198.0),
                    cyan: Color::from_rgb(139.0, 233.0, 253.0),
                    white: Color::from_rgb(249.0, 249.0, 251.0),
                },
            },
        }
    }
}

pub struct TerminalColor {
    black: Color,
    red: Color,
    green: Color,
    yellow: Color,
    blue: Color,
    magenta: Color,
    cyan: Color,
    white: Color,
}
