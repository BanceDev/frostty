//! Draw and interact with text.
pub use iced_core::text::{Fragment, Highlighter, IntoFragment, Span};
pub use iced_core::widget::text::*;

/// A bunch of text.
///
/// # Example
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::text;
/// use iced::color;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     text("Hello, this is iced!")
///         .size(20)
///         .color(color!(0x0000ff))
///         .into()
/// }
/// ```
pub type Terminal<'a, Theme = iced::Theme, Renderer = iced::Renderer> =
    iced_core::widget::Text<'a, Theme, Renderer>;
