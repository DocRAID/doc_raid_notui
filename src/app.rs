use std::{cell::RefCell, io, rc::Rc};

use ratatui::{
    layout::Alignment,
    style::{Color, Stylize},
    widgets::{Block, BorderType, Paragraph},
    Frame, Terminal,
};

use ratzilla::{
    event::{KeyCode, KeyEvent},
    DomBackend, WebRenderer,
};

#[derive(Default)]
pub struct App {
    pub path:String,
    pub counter: RefCell<u8>,

}

impl App {
    pub fn new(path:String) -> Self {
        Self {
            path,
            counter: RefCell::new(0),
        }
    }
    pub fn render(&self, frame: &mut Frame) {
        let counter = self.counter.borrow();
        let path = self.path.clone();
        let block = Block::bordered()
            .title("tui_blog")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded);

        let text = format!(
            "my blog site. powered by ratatui\n\
             Counter: {counter}\n\
             now page is {path}",
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::White)
            .bg(Color::Black)
            .centered();

        frame.render_widget(paragraph, frame.area());
    }

    pub fn handle_events(&self, key_event: KeyEvent) {
        let mut counter = self.counter.borrow_mut();
        match key_event.code {
            KeyCode::Left => *counter = counter.saturating_sub(1),
            KeyCode::Right => *counter = counter.saturating_add(1),
            _ => {}
        }
    }
}

