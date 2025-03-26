use iced::font::{Family, Stretch, Weight};
use iced::theme::Palette;
use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::{container, image, responsive, stack};
use iced::window::settings::PlatformSpecific;
use iced::{Color, Task, theme};
use iced::{Element, Fill, Font, Length, Size, Subscription};
use iced::{Theme, keyboard};
use std::collections::HashMap;
use std::env;
use terminal::TerminalView;

mod config;
mod style;
mod terminal;

const TERM_FONT_JET_BRAINS_BYTES: &[u8] =
    include_bytes!("../assets/fonts/JetBrainsMono/JetBrainsMonoNerdFontMono-Bold.ttf");

pub fn main() -> iced::Result {
    unsafe {
        env::set_var("TERM", "frostty");
    }
    iced::application("frostty", Frostty::update, Frostty::view)
        .subscription(Frostty::subscription)
        .antialiasing(false)
        .theme(Frostty::theme)
        .window(iced::window::Settings {
            platform_specific: PlatformSpecific {
                application_id: "frostty".to_string(),
                override_redirect: false,
            },
            ..Default::default()
        })
        .window_size((790.0, 460.0))
        .run_with(Frostty::new)
}

struct Frostty {
    panes: pane_grid::State<Pane>,
    terminals: HashMap<u64, terminal::Terminal>,
    term_settings: terminal::settings::Settings,
    panes_created: usize,
    focus: Option<pane_grid::Pane>,
    config: Option<config::Config>,
}

#[derive(Debug, Clone)]
enum Message {
    SplitFocused,
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Close(pane_grid::Pane),
    CloseFocused,
    Terminal(terminal::Event),
    FontLoaded(Result<(), iced::font::Error>),
}

impl Frostty {
    fn new() -> (Self, Task<Message>) {
        let (panes, pane) = pane_grid::State::new(Pane::new(0));
        let term_settings = terminal::settings::Settings {
            font: terminal::settings::FontSettings {
                size: 14.0,
                font_type: Font {
                    weight: Weight::Normal,
                    family: Family::Name("JetBrainsMono Nerd Font Mono"),
                    stretch: Stretch::Normal,
                    ..Default::default()
                },
                ..Default::default()
            },
            theme: terminal::settings::ThemeSettings::default(),
            backend: terminal::settings::BackendSettings {
                program: std::env::var("SHELL")
                    .expect("SHELL variable not defined")
                    .to_string(),
                ..Default::default()
            },
        };

        let term = terminal::Terminal::new(0, term_settings.clone());
        let mut terminals = HashMap::new();
        terminals.insert(0, term);

        (
            Frostty {
                panes,
                panes_created: 1,
                focus: Some(pane),
                terminals,
                term_settings,
                config: config::Config::new(),
            },
            Task::batch(vec![
                iced::font::load(TERM_FONT_JET_BRAINS_BYTES).map(Message::FontLoaded),
            ]),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FontLoaded(_) => {}
            Message::SplitFocused => {
                if let Some(pane) = self.focus {
                    let size = self
                        .panes
                        .layout()
                        .pane_regions(0.0, Size::new(800.0, 600.0))
                        .get(&pane)
                        .unwrap()
                        .clone();
                    let axis = if size.width >= size.height {
                        pane_grid::Axis::Vertical
                    } else {
                        pane_grid::Axis::Horizontal
                    };
                    let result = self.panes.split(axis, pane, Pane::new(self.panes_created));

                    let terminal = terminal::Terminal::new(
                        self.panes_created as u64,
                        self.term_settings.clone(),
                    );
                    let command = TerminalView::focus(terminal.widget_id());
                    self.terminals.insert(self.panes_created as u64, terminal);

                    if let Some((pane, _)) = result {
                        self.focus = Some(pane);
                    }

                    self.panes_created += 1;
                    return command;
                }
            }
            Message::FocusAdjacent(direction) => {
                if let Some(pane) = self.focus {
                    if let Some(adjacent) = self.panes.adjacent(pane, direction) {
                        let new_focused_pane = self.panes.get(adjacent).unwrap();
                        let new_focued_terminal = self
                            .terminals
                            .get_mut(&(new_focused_pane.id as u64))
                            .unwrap();
                        self.focus = Some(adjacent);
                        return TerminalView::focus(new_focued_terminal.widget_id());
                    }
                }
            }
            Message::Clicked(pane) => {
                let new_focused_pane = self.panes.get(pane).unwrap();
                let new_focued_terminal = self
                    .terminals
                    .get_mut(&(new_focused_pane.id as u64))
                    .unwrap();
                self.focus = Some(pane);
                return TerminalView::focus(new_focued_terminal.widget_id());
            }
            Message::Resized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            Message::Dragged(pane_grid::DragEvent::Dropped { pane, target }) => {
                self.panes.drop(pane, target);
            }
            Message::Dragged(_) => {}
            Message::Close(pane) => {
                if let Some((_, sibling)) = self.panes.close(pane) {
                    let new_focused_pane = self.panes.get(sibling).unwrap();
                    let new_focued_terminal = self
                        .terminals
                        .get_mut(&(new_focused_pane.id as u64))
                        .unwrap();
                    self.focus = Some(sibling);
                    return TerminalView::focus(new_focued_terminal.widget_id());
                }
            }
            Message::CloseFocused => {
                if let Some(pane) = self.focus {
                    if let Some(Pane { is_pinned, .. }) = self.panes.get(pane) {
                        if !is_pinned {
                            if let Some((_, sibling)) = self.panes.close(pane) {
                                let new_focused_pane = self.panes.get(sibling).unwrap();
                                let new_focued_terminal = self
                                    .terminals
                                    .get_mut(&(new_focused_pane.id as u64))
                                    .unwrap();
                                self.focus = Some(sibling);
                                return TerminalView::focus(new_focued_terminal.widget_id());
                            }
                        }
                    }
                }
            }
            Message::Terminal(terminal::Event::CommandReceived(id, cmd)) => {
                if let Some(terminal) = self.terminals.get_mut(&id) {
                    if terminal.update(cmd) == terminal::actions::Action::Shutdown {
                        if let Some(cur_pane) = self.focus {
                            return self.update(Message::Close(cur_pane));
                        }
                    }
                }
            }
        }

        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let mut subs = vec![];
        let key_sub = keyboard::on_key_press(|key_code, modifiers| {
            if modifiers.control() && modifiers.shift() {
                return handle_hotkey(key_code);
            }
            None
        });

        subs.push(key_sub);

        for id in self.terminals.keys() {
            let terminal = self.terminals.get(id).unwrap();
            let term_event_stream = terminal::Subscription::event_stream(terminal.id);
            subs.push(
                Subscription::run_with_id(terminal.id, term_event_stream).map(Message::Terminal),
            );
        }

        Subscription::batch(subs)
    }

    fn view(&self) -> Element<Message> {
        let focus = self.focus;

        let pane_grid = PaneGrid::new(&self.panes, |id, pane, _is_maximized| {
            let is_focused = focus == Some(id);

            pane_grid::Content::new(responsive(move |_size| {
                view_content(pane.id as u64, &self.terminals)
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

        if let Some(wallpaper) = self.config.clone().and_then(|config| config.wallpaper) {
            stack![
                image(format!("{}/.config/frostty/{}", env!("HOME"), wallpaper))
                    .content_fit(iced::ContentFit::Cover),
                container(pane_grid).padding(10)
            ]
            .width(Fill)
            .height(Fill)
            .into()
        } else {
            container(pane_grid).padding(10).into()
        }
    }

    fn theme(&self) -> Theme {
        if let Some(app) = self
            .config
            .clone()
            .and_then(|config| config.colors)
            .and_then(|colors| colors.app)
        {
            let background = app.background;
            let primary = app.active;
            let text = app.inactive;
            let theme = theme::Custom::new(
                "Config".to_string(),
                Palette {
                    background: Color::parse(&(background.unwrap_or("#1e1e2e".to_string())))
                        .expect("improperly formatted background color"),
                    primary: Color::parse(&(primary.unwrap_or("#89b4fa".to_string())))
                        .expect("improperly formatted active color"),
                    text: Color::parse(&(text.unwrap_or("#cdd6f4".to_string())))
                        .expect("improperly fomatted inactive color"),
                    ..Palette::CATPPUCCIN_MOCHA
                },
            );
            Theme::Custom(theme.into())
        } else {
            Theme::CatppuccinMocha
        }
    }
}

fn handle_hotkey(key: keyboard::Key) -> Option<Message> {
    use keyboard::key::{self, Key};
    use pane_grid::Direction;

    match key.as_ref() {
        // TODO: config file for this
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

fn view_content(
    pane_id: u64,
    terminals: &HashMap<u64, terminal::Terminal>,
) -> Element<'_, Message> {
    let terminal = terminals.get(&pane_id).expect("terminal with id not found");
    container(TerminalView::show(terminal).map(Message::Terminal))
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(5)
        .into()
}
