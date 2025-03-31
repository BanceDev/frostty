use config::Command;
use iced::font::Family;
use iced::theme::Palette;
use iced::time::{self, Instant};
use iced::widget::pane_grid::{self, PaneGrid};
use iced::widget::{container, image, responsive, stack};
use iced::window::Level;
use iced::window::settings::PlatformSpecific;
use iced::{Color, Task, theme};
use iced::{Element, Fill, Font, Length, Size, Subscription, window};
use iced::{Theme, keyboard};
use std::collections::HashMap;
use std::env;
use std::process;
use terminal::TerminalView;

mod config;
mod style;
mod terminal;

pub fn main() -> iced::Result {
    unsafe {
        env::set_var("TERM", "frostty");
    }
    let mut size = (790.0, 460.0);
    let mut level = Level::Normal;
    if let Some(win_cfg) = config::Config::new().and_then(|config| config.window) {
        if let Some(level_string) = win_cfg.level {
            if level_string == "AlwaysOnTop" {
                level = Level::AlwaysOnTop;
            }
        }

        if let Some(dim) = win_cfg.dimensions {
            size = (dim.width, dim.height);
        }
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
        .window_size(size)
        .level(level)
        .run_with(Frostty::new)
}

struct Frostty {
    panes: pane_grid::State<Pane>,
    terminals: HashMap<u64, terminal::Terminal>,
    term_settings: terminal::settings::Settings,
    panes_created: usize,
    bell: Option<pane_grid::Pane>,
    bell_len: Option<u64>,
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
    BellOn(pane_grid::Pane),
    BellOff(Instant),
    Terminal(terminal::Event),
}

impl Frostty {
    fn new() -> (Self, Task<Message>) {
        let config = config::Config::new();
        let (panes, pane) = pane_grid::State::new(Pane::new(0));
        let mut size = 14.0;
        let mut font_type = Font::MONOSPACE;
        if let Some(font) = config.clone().and_then(|config| config.font) {
            size = font.size.unwrap_or(14.0);
            font_type = match font.family {
                Some(family) => Font {
                    family: Family::Name(Box::leak(family.clone().into_boxed_str())),
                    ..Font::MONOSPACE
                },
                None => Font::MONOSPACE,
            };
        }
        let term_settings = terminal::settings::Settings {
            font: terminal::settings::FontSettings {
                size,
                font_type,
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

        let bell_len = config
            .clone()
            .and_then(|config| config.bell)
            .and_then(|bell| bell.duration);

        let mut win_mode = window::Mode::Windowed;
        let mut maximized = false;
        if let Some(mode) = config
            .clone()
            .and_then(|config| config.window)
            .and_then(|window| window.mode)
        {
            if mode == "Fullscreen" {
                win_mode = window::Mode::Fullscreen;
            }
            if mode == "Maximized" {
                maximized = true;
            }
        }

        (
            Frostty {
                panes,
                panes_created: 1,
                bell: None,
                bell_len,
                focus: Some(pane),
                terminals,
                term_settings,
                config,
            },
            Task::batch([
                iced::window::get_latest().and_then(move |id| window::change_mode(id, win_mode)),
                iced::window::get_latest().and_then(move |id| window::maximize(id, maximized)),
            ]),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
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
                if let Some((cur, sibling)) = self.panes.close(pane) {
                    self.terminals.remove(&(cur.id as u64));

                    let new_focused_pane = self.panes.get(sibling).unwrap();
                    let new_focued_terminal = self
                        .terminals
                        .get_mut(&(new_focused_pane.id as u64))
                        .unwrap();
                    self.focus = Some(sibling);
                    return TerminalView::focus(new_focued_terminal.widget_id());
                } else {
                    return window::get_latest().and_then(window::close);
                }
            }
            Message::CloseFocused => {
                if let Some(pane) = self.focus {
                    if let Some(Pane { is_pinned, .. }) = self.panes.get(pane) {
                        if !is_pinned {
                            if let Some((cur, sibling)) = self.panes.close(pane) {
                                self.terminals.remove(&(cur.id as u64));

                                let new_focused_pane = self.panes.get(sibling).unwrap();
                                let new_focued_terminal = self
                                    .terminals
                                    .get_mut(&(new_focused_pane.id as u64))
                                    .unwrap();
                                self.focus = Some(sibling);
                                return TerminalView::focus(new_focued_terminal.widget_id());
                            } else {
                                return window::get_latest().and_then(window::close);
                            }
                        }
                    }
                }
            }
            Message::BellOn(pane) => {
                if let Some(duration) = self.bell_len {
                    if duration != 0 {
                        if let Some(command) = self
                            .config
                            .clone()
                            .and_then(|config| config.bell)
                            .and_then(|bell| bell.command)
                        {
                            match command {
                                Command::Simple(program) => {
                                    process::Command::new(program)
                                        .spawn()
                                        .expect("failed to exec bell cmd");
                                }
                                Command::Complex { program, args } => {
                                    process::Command::new(program)
                                        .args(args)
                                        .spawn()
                                        .expect("failed to exec bell cmd");
                                }
                            }
                        }
                        let bell_pane = self.panes.get_mut(pane).unwrap();
                        if !bell_pane.bell {
                            bell_pane.bell = true;
                            self.bell = Some(pane);
                        }
                    }
                }
            }
            Message::BellOff(_now) => {
                if let Some(pane) = self.bell {
                    let bell_pane = self.panes.get_mut(pane).unwrap();
                    bell_pane.bell = false;
                    self.bell = None;
                }
            }
            Message::Terminal(terminal::Event::CommandReceived(id, cmd)) => {
                if let Some(terminal) = self.terminals.get_mut(&id) {
                    match terminal.update(cmd) {
                        terminal::actions::Action::Shutdown => {
                            if let Some(cur_pane) = self.focus {
                                return self.update(Message::Close(cur_pane));
                            }
                        }
                        terminal::actions::Action::Bell => {
                            if let Some(cur_pane) = self.focus {
                                return self.update(Message::BellOn(cur_pane));
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        let mut subs = vec![];
        let key_sub = keyboard::on_key_press(|key_code, modifiers| {
            if modifiers.alt() {
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
        if let Some(duration) = self.bell_len {
            if duration != 0 {
                subs.push(
                    time::every(std::time::Duration::from_millis(duration)).map(Message::BellOff),
                );
            }
        }

        Subscription::batch(subs)
    }

    fn view(&self) -> Element<Message> {
        let focus = self.focus;
        let gaps = self
            .config
            .clone()
            .and_then(|config| config.general)
            .and_then(|general| general.gaps)
            .unwrap_or(0.0);

        let pane_grid = PaneGrid::new(&self.panes, |id, pane, _is_maximized| {
            let is_focused = focus == Some(id);

            pane_grid::Content::new(responsive(move |_size| {
                view_content(pane.id as u64, pane.bell, &self.terminals)
            }))
            .style(if is_focused {
                if pane.bell {
                    style::pane_bell
                } else {
                    style::pane_focused
                }
            } else {
                style::pane_unfocused
            })
        })
        .width(Fill)
        .height(Fill)
        .spacing(gaps)
        .on_click(Message::Clicked)
        .on_drag(Message::Dragged)
        .on_resize(10, Message::Resized);

        if let Some(wallpaper) = self
            .config
            .clone()
            .and_then(|config| config.general)
            .and_then(|general| general.wallpaper)
        {
            stack![
                image(format!("{}/.config/frostty/{}", env!("HOME"), wallpaper))
                    .content_fit(iced::ContentFit::Cover),
                container(pane_grid).padding(gaps)
            ]
            .width(Fill)
            .height(Fill)
            .into()
        } else {
            container(pane_grid).padding(gaps).into()
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
                    background: Color::parse(&(background.unwrap_or("#181818".to_string())))
                        .expect("improperly formatted background color"),
                    primary: Color::parse(&(primary.unwrap_or("#ac4242".to_string())))
                        .expect("improperly formatted active color"),
                    text: Color::parse(&(text.unwrap_or("#d8d8d8".to_string())))
                        .expect("improperly fomatted inactive color"),
                    danger: Color::parse("#ac4242").expect(""),
                    success: Color::parse("#90a959").expect(""),
                },
            );
            Theme::Custom(theme.into())
        } else {
            let theme = theme::Custom::new(
                "Default".to_string(),
                Palette {
                    background: Color::parse("#181818").expect(""),
                    primary: Color::parse("#ac4242").expect(""),
                    text: Color::parse("#d8d8d8").expect("improperly fomatted inactive color"),
                    danger: Color::parse("#ac4242").expect("shouldn't fail"),
                    success: Color::parse("#90a959").expect("shouldn't fail"),
                },
            );
            Theme::Custom(theme.into())
        }
    }
}

fn handle_hotkey(key: keyboard::Key) -> Option<Message> {
    use keyboard::key::{self, Key};
    use pane_grid::Direction;

    match key.as_ref() {
        // TODO: config file for this
        Key::Character("q") => Some(Message::CloseFocused),
        Key::Character("n") => Some(Message::SplitFocused),
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
    pub bell: bool,
}

impl Pane {
    fn new(id: usize) -> Self {
        Self {
            id,
            is_pinned: false,
            bell: false,
        }
    }
}

fn view_content(
    pane_id: u64,
    bell: bool,
    terminals: &HashMap<u64, terminal::Terminal>,
) -> Element<'_, Message> {
    let terminal = terminals.get(&pane_id).expect("terminal with id not found");
    if !bell {
        container(TerminalView::show(terminal).map(Message::Terminal))
            .padding(5)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    } else {
        container(TerminalView::show(terminal).map(Message::Terminal))
            .padding(5)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
