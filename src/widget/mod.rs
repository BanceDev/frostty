pub mod terminal;
pub use terminal::Terminal;

pub fn terminal<'a, Theme, Renderer>(
    text: impl terminal::IntoFragment<'a>,
) -> Terminal<'a, Theme, Renderer>
where
    Theme: terminal::Catalog + 'a,
    Renderer: iced_core::text::Renderer,
{
    Terminal::new(text)
}

