use crate::module::router::Router;
use ratatui::layout::Layout;
use ratatui::widgets::Padding;
use ratatui::{
    layout::{Alignment, Constraint},
    style::{Color, Stylize},
    symbols,
    widgets::{Block, BorderType, Paragraph},
    Frame,
};
use ratzilla::event::{MouseEvent, MouseEventKind};
use ratzilla::{
    event::{KeyCode, KeyEvent},
    DomBackend, WebRenderer,
};
use std::{cell::RefCell, io, rc::Rc};

pub struct App {
    pub router: Router,
    pub mouse_status: RefCell<MouseEventKind>,
    pub mouse_pos: RefCell<(u32, u32)>,
    pub counter: RefCell<u8>,
}

impl App {
    pub fn new(path: String) -> Self {
        Self {
            router: Router::new(path),
            mouse_status: RefCell::new(MouseEventKind::Unidentified),
            mouse_pos: RefCell::new((0, 0)),
            counter: RefCell::new(0),
        }
    }
    pub fn render(&self, frame: &mut Frame) {
        let counter = self.counter.borrow();
        let router = self.router.clone();

        let mouse_pos = self.mouse_pos.borrow();
        let mouse_status = self.mouse_status.borrow();

        let layout = Layout::vertical([
            Constraint::Length(3), // Header
            Constraint::Max(60),   //content
        ])
        .split(frame.area());

        // Header
        let nav_block = Block::bordered()
            .border_type(BorderType::Plain)
            .padding(Padding::horizontal(1));
        let nav_paragraph = Paragraph::new("hello ● hello ● hello ● hello")
            .block(nav_block)
            .fg(Color::White)
            .bg(Color::Black)
            .centered();
        frame.render_widget(nav_paragraph, layout[0]);

        // Content
        let block = Block::bordered()
            .title(router.label())
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Plain);

        let text = format!(
            "my blog site. powered by ratatui\n\
             Counter: {counter}\n\
             now page is {:?}\n\
             \n\nmouse: {:?}, {:?}",
            router, mouse_status, mouse_pos
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::White)
            .bg(Color::Black)
            .centered();

        frame.render_widget(paragraph, layout[1]);
    }

    pub fn key_handle_events(&self, key_event: KeyEvent) {
        let mut counter = self.counter.borrow_mut();
        match key_event.code {
            KeyCode::Left => *counter = counter.saturating_sub(1),
            KeyCode::Right => *counter = counter.saturating_add(1),
            _ => {}
        }
    }
    pub fn mouse_handle_events(&self, mouse_event: MouseEvent) {
        let mut mouse_pos = self.mouse_pos.borrow_mut();
        let mut mouse_status = self.mouse_status.borrow_mut();
        *mouse_pos = (mouse_event.x, mouse_event.y);

        match mouse_event.event {
            MouseEventKind::Moved => *mouse_status = MouseEventKind::Moved,
            MouseEventKind::Pressed => *mouse_status = MouseEventKind::Pressed,
            _ => *mouse_status = MouseEventKind::Unidentified,
        }
    }
}
