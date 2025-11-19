use crate::module::mouse_tool::{calc_header_button_ranges, is_points_hovered, is_rects_hovered};
use crate::module::router::{Pages, Router};
use color_eyre::owo_colors::OwoColorize;
use log::info;
use ratatui::layout::{Layout, Position};
use ratatui::style::Style;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Padding, Wrap};
use ratatui::{
    layout::{Alignment, Constraint},
    style::{Color, Stylize},
    symbols,
    widgets::{Block, BorderType, Paragraph},
    Frame,
};
use ratzilla::event::{MouseButton, MouseEvent, MouseEventKind};
use ratzilla::web_sys::console::info;
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
static BG_RGB: Color = Color::Rgb(28, 25, 22);

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

        // Header///////////////////////////////////////////////////////

        let mut header_menu = Vec::new();

        let btn_ranges = calc_header_button_ranges(&router, layout[0].width);
        for (route,btn_range) in router.nav_bar().iter().zip(btn_ranges.iter()) {
            let mut btn_color = Color::White;
            if is_points_hovered(btn_range.0 as u16 ,btn_range.1 as u16,1,2,*mouse_pos) {
                btn_color = Color::Green;
                if *mouse_status == MouseEventKind::Pressed {
                    //todo: redirect
                }
                info!("{:?}, {}",btn_range, layout[0].width);
            }
            header_menu.push(Span::styled(
                format!(" [{}] ", route.to_string()),
                Style::new().fg(btn_color),
            ));
        }

        let header_block = Block::bordered()
            .border_type(BorderType::Plain)
            .padding(Padding::horizontal(1));


        let header_paragraph = Paragraph::new(Line::from(header_menu))
            .block(header_block)
            .fg(Color::White)
            .bg(BG_RGB)
            .centered();

        frame.render_widget(header_paragraph, layout[0]);
        // Content///////////////////////////////////////////////////////
        let block = Block::bordered()
            .title(format!("[{}]", router.label()))
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Plain);

        let text = format!(
            "my blog site. powered by ratatui\n\
             Counter: {counter}\n\
             now page is {:?}\n\
             \n\nmouse: {:?}, {:?}\
             ",
            router, mouse_status, mouse_pos
        );

        let paragraph = Paragraph::new(text)
            .block(block)
            .fg(Color::White)
            .bg(BG_RGB)
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
