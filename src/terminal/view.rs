use crate::terminal::backend::{
    Backend, BackendCommand, LinkAction, MouseButton, RenderableContent,
};
use crate::terminal::bindings::{BindingAction, BindingsLayout, InputKind};
use crate::terminal::theme::TerminalStyle;
use crate::terminal::{Command, Event, Terminal};
use alacritty_terminal::index::Point as TerminalGridPoint;
use alacritty_terminal::selection::SelectionType;
use alacritty_terminal::term::{TermMode, cell};
use alacritty_terminal::vte::ansi::CursorShape;
use iced::alignment::{Horizontal, Vertical};
use iced::mouse::{Cursor, ScrollDelta};
use iced::widget::canvas::{Path, Text};
use iced::widget::container;
use iced::{Element, Length, Point, Rectangle, Size, Theme};
use iced_core::clipboard::Kind as ClipboardKind;
use iced_core::keyboard::{Key, Modifiers};
use iced_core::mouse::{self, Click};
use iced_core::text::{LineHeight, Shaping};
use iced_core::widget::operation;
use iced_graphics::core::Widget;
use iced_graphics::core::widget::{Tree, tree};
use iced_graphics::geometry::Stroke;

pub struct TerminalView<'a> {
    term: &'a Terminal,
}

impl<'a> TerminalView<'a> {
    pub fn show(term: &'a Terminal) -> Element<'a, Event> {
        container(Self { term })
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| term.theme.container_style())
            .into()
    }

    pub fn focus<Message: 'static>(id: iced::widget::text_input::Id) -> iced::Task<Message> {
        iced::widget::text_input::focus(id)
    }

    fn is_cursor_in_layout(&self, cursor: Cursor, layout: iced_graphics::core::Layout<'_>) -> bool {
        if let Some(cursor_position) = cursor.position() {
            let layout_position = layout.position();
            let layout_size = layout.bounds();
            let is_triggered = cursor_position.x >= layout_position.x
                && cursor_position.y >= layout_position.y
                && cursor_position.x < (layout_position.x + layout_size.width)
                && cursor_position.y < (layout_position.y + layout_size.height);

            return is_triggered;
        }

        false
    }

    fn is_cursor_hovered_hyperlink(&self, state: &TerminalViewState) -> bool {
        if let Some(ref backend) = self.term.backend {
            let content = backend.renderable_content();
            if let Some(hyperlink_range) = &content.hovered_hyperlink {
                return hyperlink_range.contains(&state.mouse_position_on_grid);
            }
        }

        false
    }

    fn handle_mouse_event(
        &self,
        state: &mut TerminalViewState,
        layout_position: Point,
        cursor_position: Point,
        event: iced::mouse::Event,
    ) -> Vec<Command> {
        let mut commands = Vec::new();
        if let Some(backend) = &self.term.backend {
            let terminal_content = backend.renderable_content();
            let terminal_mode = terminal_content.terminal_mode;

            match event {
                iced_core::mouse::Event::ButtonPressed(iced_core::mouse::Button::Left) => {
                    Self::handle_left_button_pressed(
                        state,
                        &terminal_mode,
                        cursor_position,
                        layout_position,
                        &mut commands,
                    );
                }
                iced_core::mouse::Event::CursorMoved { position } => {
                    Self::handle_cursor_moved(
                        state,
                        backend.renderable_content(),
                        position,
                        layout_position,
                        &mut commands,
                    );
                }
                iced_core::mouse::Event::ButtonReleased(iced_core::mouse::Button::Left) => {
                    Self::handle_button_released(
                        state,
                        &terminal_mode,
                        &self.term.bindings,
                        &mut commands,
                    );
                }
                iced::mouse::Event::WheelScrolled { delta } => {
                    Self::handle_wheel_scrolled(
                        state,
                        delta,
                        &self.term.font.measure,
                        &mut commands,
                    );
                }
                _ => {}
            }
        }

        commands
    }

    fn handle_left_button_pressed(
        state: &mut TerminalViewState,
        terminal_mode: &TermMode,
        cursor_position: Point,
        layout_position: Point,
        commands: &mut Vec<Command>,
    ) {
        let cmd = if terminal_mode.intersects(TermMode::MOUSE_MODE) {
            Command::ProcessBackendCommand(BackendCommand::MouseReport(
                MouseButton::LeftButton,
                state.keyboard_modifiers,
                state.mouse_position_on_grid,
                true,
            ))
        } else {
            let current_click = Click::new(cursor_position, mouse::Button::Left, state.last_click);
            let selection_type = match current_click.kind() {
                mouse::click::Kind::Single => SelectionType::Simple,
                mouse::click::Kind::Double => SelectionType::Semantic,
                mouse::click::Kind::Triple => SelectionType::Lines,
            };
            state.last_click = Some(current_click);
            Command::ProcessBackendCommand(BackendCommand::SelectStart(
                selection_type,
                (
                    cursor_position.x - layout_position.x,
                    cursor_position.y - layout_position.y,
                ),
            ))
        };
        commands.push(cmd);
        state.is_dragged = true;
    }

    fn handle_cursor_moved(
        state: &mut TerminalViewState,
        terminal_content: &RenderableContent,
        position: Point,
        layout_position: Point,
        commands: &mut Vec<Command>,
    ) {
        let cursor_x = position.x - layout_position.x;
        let cursor_y = position.y - layout_position.y;
        state.mouse_position_on_grid = Backend::selection_point(
            cursor_x,
            cursor_y,
            &terminal_content.terminal_size,
            terminal_content.grid.display_offset(),
        );

        // Handle command or selection update based on terminal mode and modifiers
        if state.is_dragged {
            let terminal_mode = terminal_content.terminal_mode;
            let cmd = if terminal_mode.intersects(TermMode::MOUSE_MOTION) {
                Command::ProcessBackendCommand(BackendCommand::MouseReport(
                    MouseButton::LeftMove,
                    state.keyboard_modifiers,
                    state.mouse_position_on_grid,
                    true,
                ))
            } else {
                Command::ProcessBackendCommand(BackendCommand::SelectUpdate((cursor_x, cursor_y)))
            };
            commands.push(cmd);
        }

        // Handle link hover if applicable
        if state.keyboard_modifiers == Modifiers::COMMAND {
            commands.push(Command::ProcessBackendCommand(BackendCommand::ProcessLink(
                LinkAction::Hover,
                state.mouse_position_on_grid,
            )));
        }
    }

    fn handle_button_released(
        state: &mut TerminalViewState,
        terminal_mode: &TermMode,
        bindings: &BindingsLayout, // Use the actual type of your bindings here
        commands: &mut Vec<Command>,
    ) {
        state.is_dragged = false;

        if terminal_mode.intersects(TermMode::MOUSE_MODE) {
            commands.push(Command::ProcessBackendCommand(BackendCommand::MouseReport(
                MouseButton::LeftButton,
                state.keyboard_modifiers,
                state.mouse_position_on_grid,
                false,
            )));
        }

        if bindings.get_action(
            InputKind::Mouse(iced_core::mouse::Button::Left),
            state.keyboard_modifiers,
            *terminal_mode,
        ) == BindingAction::LinkOpen
        {
            commands.push(Command::ProcessBackendCommand(BackendCommand::ProcessLink(
                LinkAction::Open,
                state.mouse_position_on_grid,
            )));
        }
    }

    fn handle_wheel_scrolled(
        state: &mut TerminalViewState,
        delta: ScrollDelta,
        font_measure: &Size<f32>,
        commands: &mut Vec<Command>,
    ) {
        match delta {
            ScrollDelta::Lines { y, .. } => {
                let lines = y.signum() * y.abs().round();
                commands.push(Command::ProcessBackendCommand(BackendCommand::Scroll(
                    lines as i32,
                )));
            }
            ScrollDelta::Pixels { y, .. } => {
                state.scroll_pixels -= y;
                let line_height = font_measure.height; // Assume this method exists and gives the height of a line
                let lines = (state.scroll_pixels / line_height).trunc();
                state.scroll_pixels %= line_height;
                if lines != 0.0 {
                    commands.push(Command::ProcessBackendCommand(BackendCommand::Scroll(
                        lines as i32,
                    )));
                }
            }
        }
    }

    fn handle_keyboard_event(
        &self,
        state: &mut TerminalViewState,
        clipboard: &mut dyn iced_graphics::core::Clipboard,
        event: iced::keyboard::Event,
    ) -> Option<Command> {
        if let Some(backend) = &self.term.backend {
            let mut binding_action = BindingAction::Ignore;
            let last_content = backend.renderable_content();
            match event {
                iced::keyboard::Event::ModifiersChanged(m) => {
                    state.keyboard_modifiers = m;
                    let action = if state.keyboard_modifiers == Modifiers::COMMAND {
                        LinkAction::Hover
                    } else {
                        LinkAction::Clear
                    };
                    return Some(Command::ProcessBackendCommand(BackendCommand::ProcessLink(
                        action,
                        state.mouse_position_on_grid,
                    )));
                }
                iced::keyboard::Event::KeyPressed {
                    key,
                    modifiers,
                    text,
                    ..
                } => match key {
                    Key::Character(_) => {
                        if let Some(c) = text {
                            binding_action = self.term.bindings.get_action(
                                InputKind::Char(c.to_ascii_lowercase()),
                                modifiers,
                                last_content.terminal_mode,
                            );

                            if binding_action == BindingAction::Ignore && !modifiers.alt() {
                                return Some(Command::ProcessBackendCommand(
                                    BackendCommand::Write(c.as_bytes().to_vec()),
                                ));
                            }
                        }
                    }
                    Key::Named(code) => {
                        binding_action = self.term.bindings.get_action(
                            InputKind::KeyCode(code),
                            modifiers,
                            last_content.terminal_mode,
                        );
                    }
                    _ => {}
                },
                _ => {}
            }

            match binding_action {
                BindingAction::Char(c) => {
                    let mut buf = [0, 0, 0, 0];
                    let str = c.encode_utf8(&mut buf);
                    return Some(Command::ProcessBackendCommand(BackendCommand::Write(
                        str.as_bytes().to_vec(),
                    )));
                }
                BindingAction::Esc(seq) => {
                    return Some(Command::ProcessBackendCommand(BackendCommand::Write(
                        seq.as_bytes().to_vec(),
                    )));
                }
                BindingAction::Paste => {
                    if let Some(data) = clipboard.read(ClipboardKind::Standard) {
                        let input: Vec<u8> = data.bytes().collect();
                        return Some(Command::ProcessBackendCommand(BackendCommand::Write(input)));
                    }
                }
                BindingAction::Copy => {
                    clipboard.write(ClipboardKind::Standard, backend.selectable_content());
                }
                // TODO: Can handle implementing the bell in the same way here
                _ => {}
            };
        }

        None
    }
}

impl Widget<Event, Theme, iced::Renderer> for TerminalView<'_> {
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<TerminalViewState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(TerminalViewState::new())
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &iced_core::layout::Limits,
    ) -> iced_core::layout::Node {
        let size = limits.resolve(Length::Fill, Length::Fill, Size::ZERO);
        iced::advanced::layout::Node::new(size)
    }

    fn operate(
        &self,
        tree: &mut Tree,
        _layout: iced_core::Layout<'_>,
        _renderer: &iced::Renderer,
        operation: &mut dyn operation::Operation,
    ) {
        let state = tree.state.downcast_mut::<TerminalViewState>();
        let wid = iced_core::widget::Id::from(self.term.widget_id());
        operation.focusable(state, Some(&wid));
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: iced::advanced::Layout,
        _cursor: Cursor,
        viewport: &Rectangle,
    ) {
        if let Some(backend) = &self.term.backend {
            let state = tree.state.downcast_ref::<TerminalViewState>();
            let content = backend.renderable_content();
            let term_size = content.terminal_size;
            let cell_width = term_size.cell_width as f32;
            let cell_height = term_size.cell_height as f32;
            let font_size = self.term.font.size;
            let font_scale_factor = self.term.font.scale_factor;
            let layout_offset_x = layout.position().x;
            let layout_offset_y = layout.position().y;

            let geom = self.term.cache.draw(renderer, viewport.size(), |frame| {
                for indexed in content.grid.display_iter() {
                    let x = layout_offset_x + (indexed.point.column.0 as f32 * cell_width);
                    let y = layout_offset_y
                        + ((indexed.point.line.0 as f32 + content.grid.display_offset() as f32)
                            * cell_height);

                    let mut fg = self.term.theme.get_color(indexed.fg);
                    let mut bg = self.term.theme.get_color(indexed.bg);

                    // Handle dim, inverse, and selected text
                    if indexed
                        .cell
                        .flags
                        .intersects(cell::Flags::DIM | cell::Flags::DIM_BOLD)
                    {
                        fg.a *= 0.7;
                    }
                    if indexed.cell.flags.contains(cell::Flags::INVERSE)
                        || content
                            .selectable_range
                            .map_or(false, |r| r.contains(indexed.point))
                    {
                        std::mem::swap(&mut fg, &mut bg);
                    }

                    let cell_size = Size::new(cell_width, cell_height);

                    // Draw cell background
                    let background = Path::rectangle(Point::new(x, y), cell_size);
                    frame.fill(&background, bg);

                    // Draw hovered hyperlink underline
                    if content.hovered_hyperlink.as_ref().map_or(false, |range| {
                        range.contains(&indexed.point)
                            && range.contains(&state.mouse_position_on_grid)
                    }) {
                        let underline_height = y + cell_size.height;
                        let underline = Path::line(
                            Point::new(x, underline_height),
                            Point::new(x + cell_size.width, underline_height),
                        );
                        frame.stroke(
                            &underline,
                            Stroke::default()
                                .with_width(font_size * 0.15)
                                .with_color(fg),
                        );
                    }

                    // Handle cursor rendering
                    if content.grid.cursor.point == indexed.point {
                        let cursor_color = self.term.theme.get_color(content.cursor.fg);
                        let cursor_size = match content.cursor_style.shape {
                            CursorShape::Block => cell_size,
                            CursorShape::Beam => Size::new(1.0, cell_height),
                            CursorShape::Underline => Size::new(cell_width, 1.0),
                            CursorShape::HollowBlock => cell_size,
                            CursorShape::Hidden => Size::new(0.0, 0.0),
                        };
                        let cursor_shape = match content.cursor_style.shape {
                            CursorShape::Underline => {
                                Path::rectangle(Point::new(x, y + cell_height - 1.0), cursor_size)
                            }
                            CursorShape::HollowBlock => Path::new(|b| {
                                b.move_to(Point::new(x, y));
                                b.line_to(Point::new(cell_width, y));
                                b.line_to(Point::new(cell_width, cell_height));
                                b.line_to(Point::new(x, cell_height));
                                b.line_to(Point::new(x, y));
                            }),
                            _ => Path::rectangle(Point::new(x, y), cursor_size),
                        };
                        frame.fill(&cursor_shape, cursor_color);
                    }

                    // Draw text
                    if indexed.c != ' ' && indexed.c != '\t' {
                        if content.grid.cursor.point == indexed.point
                            && content.terminal_mode.contains(TermMode::APP_CURSOR)
                            && content.cursor_style.shape == CursorShape::Block
                        {
                            fg = bg;
                        }
                        let text = Text {
                            content: indexed.c.to_string(),
                            position: Point::new(
                                x + (cell_size.width / 2.0),
                                y + (cell_size.height / 2.0),
                            ),
                            font: self.term.font.font_type,
                            size: iced_core::Pixels(font_size),
                            color: fg,
                            horizontal_alignment: Horizontal::Center,
                            vertical_alignment: Vertical::Center,
                            shaping: Shaping::Advanced,
                            line_height: LineHeight::Relative(font_scale_factor),
                        };
                        frame.fill_text(text);
                    }
                }
            });

            use iced::advanced::graphics::geometry::Renderer as _;
            renderer.draw_geometry(geom);
        }
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: iced::Event,
        layout: iced_graphics::core::Layout<'_>,
        cursor: Cursor,
        _renderer: &iced::Renderer,
        clipboard: &mut dyn iced_graphics::core::Clipboard,
        shell: &mut iced_graphics::core::Shell<'_, Event>,
        _viewport: &Rectangle,
    ) -> iced::event::Status {
        let state = tree.state.downcast_mut::<TerminalViewState>();
        let layout_size = layout.bounds().size();
        if state.size != layout_size && self.term.backend.is_some() {
            state.size = layout_size;
            let cmd =
                Command::ProcessBackendCommand(BackendCommand::Resize(Some(layout_size), None));
            shell.publish(Event::CommandReceived(self.term.id, cmd));
        }

        if !state.is_focused {
            state.keyboard_modifiers = Modifiers::empty();
            return iced::event::Status::Ignored;
        }

        let commands = match event {
            iced::Event::Mouse(mouse_event) if self.is_cursor_in_layout(cursor, layout) => {
                self.handle_mouse_event(
                    state,
                    layout.position(),
                    cursor.position().unwrap(), // Assuming cursor position is always available here.
                    mouse_event,
                )
            }
            iced::Event::Keyboard(keyboard_event) => {
                self.handle_keyboard_event(state, clipboard, keyboard_event)
                    .into_iter() // Convert Option to iterator (0 or 1 element)
                    .collect()
            }
            _ => Vec::new(), // No commands for other events.
        };

        if !commands.is_empty() {
            for cmd in commands {
                shell.publish(Event::CommandReceived(self.term.id, cmd));
            }
            // WARN: not sure if this is wise yet
            iced::event::Status::Ignored
        } else {
            iced::event::Status::Ignored
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: iced_core::Layout<'_>,
        cursor: iced_core::mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> iced_core::mouse::Interaction {
        let state = tree.state.downcast_ref::<TerminalViewState>();
        let mut cursor_mode = iced_core::mouse::Interaction::Idle;
        let mut terminal_mode = TermMode::empty();
        if let Some(backend) = &self.term.backend {
            terminal_mode = backend.renderable_content().terminal_mode;
        }
        if self.is_cursor_in_layout(cursor, layout) && !terminal_mode.contains(TermMode::SGR_MOUSE)
        {
            cursor_mode = iced_core::mouse::Interaction::Text;
        }

        if self.is_cursor_hovered_hyperlink(state) {
            cursor_mode = iced_core::mouse::Interaction::Pointer;
        }

        cursor_mode
    }
}

impl<'a> From<TerminalView<'a>> for Element<'a, Event, Theme, iced::Renderer> {
    fn from(widget: TerminalView<'a>) -> Self {
        Self::new(widget)
    }
}

#[derive(Debug, Clone)]
struct TerminalViewState {
    is_focused: bool,
    is_dragged: bool,
    last_click: Option<mouse::Click>,
    scroll_pixels: f32,
    keyboard_modifiers: Modifiers,
    size: Size<f32>,
    mouse_position_on_grid: TerminalGridPoint,
}

impl TerminalViewState {
    fn new() -> Self {
        Self {
            is_focused: true,
            is_dragged: false,
            last_click: None,
            scroll_pixels: 0.0,
            keyboard_modifiers: Modifiers::empty(),
            size: Size::from([0.0, 0.0]),
            mouse_position_on_grid: TerminalGridPoint::default(),
        }
    }
}

impl Default for TerminalViewState {
    fn default() -> Self {
        Self::new()
    }
}

impl operation::Focusable for TerminalViewState {
    fn is_focused(&self) -> bool {
        self.is_focused
    }

    fn focus(&mut self) {
        self.is_focused = true;
    }

    fn unfocus(&mut self) {
        self.is_focused = false;
    }
}
