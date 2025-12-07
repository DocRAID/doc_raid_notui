use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Color, Stylize};
use ratatui::widgets::{Block, BorderType, HighlightSpacing, List, Paragraph};
use ratatui::Frame;
use ratatui::style::{Modifier, Style};

pub fn blog_page(label: String, frame: &mut Frame, layout: Rect) {
    let block = Block::bordered()
        .title(format!("{{ {} }}", label))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    let text = "my blog site. powered by ratatui\n\
             now page is \n\
             ";

    let paragraph = Paragraph::new(text)
        .block(block)
        .fg(Color::White)
        .bg(crate::app::BG_RGB)
        .centered();
    // tag
    let tag_block = Block::bordered()
        .title("{{ tags }}")
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    //todo: module에서 가져오게 하기
    let tags_vec = vec!["linux","gcc","knowledge"];

    let tag_list = List::new(tags_vec)
        .block(tag_block)
        .bg(crate::app::BG_RGB)
        .highlight_style(Style::new().bg(Color::White).add_modifier(Modifier::BOLD))
        .highlight_symbol(">")
        .highlight_spacing(HighlightSpacing::Always);

    let split_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(15),
            Constraint::Percentage(85),
        ])
        .split(layout);

    frame.render_widget(paragraph, split_layout[1]);
    frame.render_widget(tag_list, split_layout[0]);
}
/*
    title, tags, date, url

    click url
*/