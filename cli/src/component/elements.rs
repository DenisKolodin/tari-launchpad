use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders, Paragraph};

pub fn block_with_title(title: Option<&str>) -> Block<'_> {
    let block = Block::default().borders(Borders::ALL);
    if let Some(title) = title {
        let title = format!(" {title} ");
        let style = Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);
        let title_span = Span::styled(title, style);
        block.title(title_span)
    } else {
        block
    }
}

pub fn logo(logo: &str) -> Paragraph<'_> {
    let text = logo.trim_start_matches(char::is_whitespace);
    Paragraph::new(text)
}
