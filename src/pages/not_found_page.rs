use ratatui::layout::{Alignment, Rect};
use ratatui::prelude::{Color, Stylize};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::Frame;

pub fn not_found_page(label: String, frame: &mut Frame, layout: Rect) {
    let block = Block::bordered()
        .title(format!("{{ {} }}", label))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Plain);

    // self.router
    let text = r#"



+=======================================================================================+
|                                                                                       |
|  _  _      ___    _  _                                                                |
| | || |    / _ \  | || |                                                               |
| | || |_  | | | | | || |_                                                              |
| |__   _| | | | | |__   _|                                                             |
|    | |   | |_| |    | |                                                               |
|    |_|    \___/     |_|                                                               |
|                                                                                       |
| .__   __.   ______   .__________.  _______   ______    __    __  .__   __.  _______   |
| |  \ |  |  /  __  \  |          | |   ____| /  __  \  |  |  |  | |  \ |  | |       \  |
| |   \|  | |  |  |  | `---|  |---` |  |___  |  |  |  | |  |  |  | |   \|  | |  .--.  | |
| |  . `  | |  |  |  |     |  |     |   ___| |  |  |  | |  |  |  | |  . `  | |  |  |  | |
| |  |\   | |  `--'  |     |  |     |  |     |  `--'  | |  `--'  | |  |\   | |  '--'  | |
| |__| \__|  \______/      |__|     |__|      \______/   \______/  |__| \__| |_______/  |
|                                                                                       |
+=======================================================================================+
"#;

    let paragraph = Paragraph::new(text)
        .block(block)
        .fg(Color::White)
        .bg(crate::app::BG_RGB)
        .centered();

    frame.render_widget(paragraph, layout);
}
