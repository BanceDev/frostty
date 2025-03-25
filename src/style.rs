use iced::widget::container;
use iced::{Border, Theme};

pub fn pane_active(theme: &Theme) -> container::Style {
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
