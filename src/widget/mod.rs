pub mod terminal;
pub use terminal::Terminal;
pub use terminal::{TerminalPalette, TerminalTheme};

pub fn terminal<'a, Theme, Renderer>(
    text: impl terminal::IntoFragment<'a>,
) -> Terminal<'a, Theme, Renderer>
where
    Theme: terminal::Catalog + 'a,
    Renderer: iced_core::text::Renderer,
{
    Terminal::new(text)
}

pub fn rich_text<'a, Link, Theme, Renderer>(
    spans: impl AsRef<[terminal::Span<'a, Link, Renderer::Font>]> + 'a,
) -> terminal::Rich<'a, Link, Theme, Renderer>
where
    Link: Clone + 'static,
    Theme: terminal::Catalog + 'a,
    Renderer: iced_core::text::Renderer,
    Renderer::Font: 'a,
{
    terminal::Rich::with_spans(spans)
}

pub fn terminal_palette(theme: TerminalTheme) -> TerminalPalette {
    theme.get_theme()
}
