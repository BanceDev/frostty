use iced::widget::container;
use iced::{Background, Border, Color, Theme};

use crate::config;

pub fn pane_unfocused(theme: &Theme) -> container::Style {
    let palette = theme.palette();
    let config = config::Config::new();
    let radius = match config
        .clone()
        .and_then(|config| config.border)
        .and_then(|border| border.radius)
    {
        Some(rad) => iced::border::Radius::new(rad),
        None => iced::border::Radius::default(),
    };
    let width = match config
        .clone()
        .and_then(|config| config.border)
        .and_then(|border| border.thickness)
    {
        Some(width) => width,
        None => 2.0,
    };

    container::Style {
        background: None,
        border: Border {
            width,
            color: palette.text,
            radius,
        },
        ..Default::default()
    }
}

pub fn pane_focused(theme: &Theme) -> container::Style {
    let palette = theme.palette();
    let config = config::Config::new();
    let radius = match config
        .clone()
        .and_then(|config| config.border)
        .and_then(|border| border.radius)
    {
        Some(rad) => iced::border::Radius::new(rad),
        None => iced::border::Radius::default(),
    };
    let width = match config
        .clone()
        .and_then(|config| config.border)
        .and_then(|border| border.thickness)
    {
        Some(width) => width,
        None => 2.0,
    };

    container::Style {
        background: None,
        border: Border {
            width,
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
