use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders};

pub fn block_with_title(title: &str) -> Block<'_> {
    let title = format!(" {title} ");
    let style = Style::default()
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);
    let title_span = Span::styled(title, style);
    Block::default().borders(Borders::ALL) //.title(title_span)
}
