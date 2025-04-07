use iced::widget::container;
use iced::{Background, Border, Color, Theme};

use crate::config;

pub fn pane_unfocused(theme: &Theme) -> container::Style {
    let palette = theme.palette();
    let radius = match config::Config::new()
        .and_then(|config| config.general)
        .and_then(|general| general.radius)
    {
        Some(rad) => iced::border::Radius::new(rad),
        None => iced::border::Radius::default(),
    };

    container::Style {
        background: None,
        border: Border {
            width: 2.0,
            color: palette.text,
            radius,
        },
        ..Default::default()
    }
}

pub fn pane_focused(theme: &Theme) -> container::Style {
    let palette = theme.palette();
    let radius = match config::Config::new()
        .and_then(|config| config.general)
        .and_then(|general| general.radius)
    {
        Some(rad) => iced::border::Radius::new(rad),
        None => iced::border::Radius::default(),
    };

    container::Style {
        background: None,
        border: Border {
            width: 2.0,
            color: palette.primary,
            radius,
        },
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
