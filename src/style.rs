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
pub fn tab_unfocused(theme: &Theme) -> container::Style {
    let palette = theme.palette();

    container::Style {
        background: Some(Background::from(palette.background)),
        border: Border {
            width: 2.0,
            color: palette.text,
            ..Border::default()
        },
        ..Default::default()
    }
}

pub fn tab_focused(theme: &Theme) -> container::Style {
    let palette = theme.palette();

    container::Style {
        background: Some(Background::from(palette.primary)),
        border: Border {
            width: 2.0,
            color: palette.primary,
            ..Border::default()
        },
        text_color: Some(palette.background),
        ..Default::default()
    }
}

pub fn pane_bell(_theme: &Theme) -> container::Style {
    let background = match config::Config::new()
        .and_then(|config| config.bell)
        .and_then(|bell| bell.color)
    {
        Some(color) => {
            let bg = Color::parse(&color).expect("improperly formatted bell color");
            Some(Background::from(bg))
        }
        None => None,
    };

    container::Style {
        background,
        ..Default::default()
    }
}
