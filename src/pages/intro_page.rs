use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Color, Stylize};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::Frame;

pub fn intro_page(label: String, frame: &mut Frame, layout: Rect) {
    let block = Block::bordered()
        .title(format!("{{ {} }}", label))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    // self.router
    let text = "my blog site. powered by ratatui\n\
             now page is \n\
             ";

    let paragraph = Paragraph::new(text)
        .block(block)
        .fg(Color::White)
        .bg(crate::app::BG_RGB)
        .centered();

    frame.render_widget(paragraph, layout);
}
