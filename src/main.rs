use iced::{keyboard, Theme};
use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::{
    container, responsive, scrollable,
};
use iced::{Element, Fill, Size, Subscription};
use crate::widget::terminal;

mod style;
mod widget;

pub fn main() -> iced::Result {
    iced::application("Frostty", Frostty::update, Frostty::view)
        .subscription(Frostty::subscription)
        .theme(Frostty::theme)
        .run()
}

struct Frostty {
    panes: pane_grid::State<Pane>,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused,
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Close(pane_grid::Pane),
    CloseFocused,
}

impl Frostty {
    fn new() -> Self {
        let (panes, _) = pane_grid::State::new(Pane::new(0));

        Frostty {
            panes,
            panes_created: 1,
            focus: None,
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Split(axis, pane) => {
                let result =
                    self.panes.split(axis, pane, Pane::new(self.panes_created));

                if let Some((pane, _)) = result {
                    self.focus = Some(pane);
                }

                self.panes_created += 1;
            }
            Message::SplitFocused => {
                if let Some(pane) = self.focus {
                    let size = self.panes.layout()
                        .pane_regions(0.0, Size::new(800.0, 600.0))
                        .get(&pane)
                        .unwrap()
                        .clone();
                    let axis = if size.width >= size.height { pane_grid::Axis::Vertical } 
                            else { pane_grid::Axis::Horizontal };
                    let result = self.panes.split(
                        axis,
                        pane,
                        Pane::new(self.panes_created),
                    );

                    if let Some((pane, _)) = result {
                        self.focus = Some(pane);
                    }

                    self.panes_created += 1;
                }
            }
            Message::FocusAdjacent(direction) => {
                if let Some(pane) = self.focus {
                    if let Some(adjacent) = self.panes.adjacent(pane, direction)
                    {
                        self.focus = Some(adjacent);
                    }
                }
            }
            Message::Clicked(pane) => {
                self.focus = Some(pane);
            }
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            Message::Dragged(pane_grid::DragEvent::Dropped {
                pane,
                target,
            }) => {
                self.panes.drop(pane, target);
            }
            Message::Dragged(_) => {}
            Message::Close(pane) => {
                if let Some((_, sibling)) = self.panes.close(pane) {
                    self.focus = Some(sibling);
                }
            }
            Message::CloseFocused => {
                if let Some(pane) = self.focus {
                    if let Some(Pane { is_pinned, .. }) = self.panes.get(pane) {
                        if !is_pinned {
                            if let Some((_, sibling)) = self.panes.close(pane) {
                                self.focus = Some(sibling);
                            }
                        }
                    }
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        keyboard::on_key_press(|key_code, modifiers| {
            if !modifiers.command() {
                return None;
            }

            handle_hotkey(key_code)
        })
    }

    fn view(&self) -> Element<Message> {
        let focus = self.focus;

        let pane_grid = PaneGrid::new(&self.panes, |id, pane, _is_maximized| {
            let is_focused = focus == Some(id);

            pane_grid::Content::new(responsive(move |_size| {
                view_content(pane)
            }))
            .style(if is_focused {
                style::pane_focused
            } else {
                style::pane_active
            })
        })
        .width(Fill)
        .height(Fill)
        .spacing(10)
        .on_click(Message::Clicked)
        .on_drag(Message::Dragged)
        .on_resize(10, Message::Resized);

        container(pane_grid).padding(10).into()
    }

    fn theme(&self) -> Theme {
        Theme::CatppuccinMocha
    }
}

impl Default for Frostty {
    fn default() -> Self {
        Frostty::new()
    }
}

fn handle_hotkey(key: keyboard::Key) -> Option<Message> {
    use keyboard::key::{self, Key};
    use pane_grid::Direction;

    match key.as_ref() {
        Key::Character("n") => Some(Message::SplitFocused),
        Key::Character("q") => Some(Message::CloseFocused),
        Key::Named(key) => {
            let direction = match key {
                key::Named::ArrowUp => Some(Direction::Up),
                key::Named::ArrowDown => Some(Direction::Down),
                key::Named::ArrowLeft => Some(Direction::Left),
                key::Named::ArrowRight => Some(Direction::Right),
                _ => None,
            };

            direction.map(Message::FocusAdjacent)
        }
        _ => None,
    }
}

#[derive(Clone, Copy)]
struct Pane {
    id: usize,
    pub is_pinned: bool,
}

impl Pane {
    fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
        }
    }
}

fn view_content<'a>(
    pane: &Pane,
) -> Element<'a, Message> {

    let content = terminal(pane.id.to_string()).size(16);

    container(scrollable(content)).padding(5).into()
}
