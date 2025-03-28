use iced::widget::container;
use iced::{Background, Border, Theme};

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

    container::Style {
        background: Some(Background::from(palette.text)),
        ..Default::default()
    }
}
