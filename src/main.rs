use iced::{keyboard, Theme};
use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::{
    button, center, column, container, responsive, scrollable, text,
};
use iced::{Center, Color, Element, Fill, Size, Subscription};

mod style;

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
        let total_panes = self.panes.len();

        let pane_grid = PaneGrid::new(&self.panes, |id, pane, _is_maximized| {
            let is_focused = focus == Some(id);

            pane_grid::Content::new(responsive(move |size| {
                view_content(id, total_panes, pane.is_pinned, size)
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
    pane: pane_grid::Pane,
    total_panes: usize,
    is_pinned: bool,
    size: Size,
) -> Element<'a, Message> {
    let button = |label, message| {
        button(text(label).width(Fill).align_x(Center).size(16))
            .width(Fill)
            .padding(8)
            .on_press(message)
    };

    let controls = column![]
    .push_maybe(if total_panes > 1 && !is_pinned {
        Some(button("Close", Message::Close(pane)).style(button::danger))
    } else {
        None
    })
    .spacing(5)
    .max_width(160);

    let content =
        column![text!("{}x{}", size.width, size.height).size(24), controls,]
            .spacing(10)
            .align_x(Center);

    center(scrollable(content)).padding(5).into()
}
