use iced::widget::container;
use iced::{Background, Border, Color, Theme};

use crate::config;

pub fn pane_unfocused(theme: &Theme) -> container::Style {
    let palette = theme.palette();

    container::Style {
        background: None,
        border: Border {
            width: 2.0,
            color: palette.text,
            ..Border::default()
        },
        ..Default::default()
    }
}

pub fn pane_focused(theme: &Theme) -> container::Style {
    let palette = theme.palette();

    container::Style {
        background: None,
        border: Border {
            width: 2.0,
            color: palette.primary,
            ..Border::default()
        },
        ..Default::default()
    }
}

pub fn pane_bell(theme: &Theme) -> container::Style {
    let palette = theme.palette();

    let background = match config::Config::new()
        .and_then(|config| config.bell)
        .and_then(|bell| bell.color)
    {
        Some(color) => {
            let bg = Color::parse(&color).expect("improperly formatted bell color");
            Some(Background::from(bg))
        }
        None => Some(Background::from(palette.text)),
    };

    container::Style {
        background,
        ..Default::default()
    }
}
