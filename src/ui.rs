use crossterm::style::Attribute::Bold;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};

pub fn ui1<B: Backend>(f: &mut Frame<B>, url: &str, border: i32, folder: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
        .split(f.size());
    let text = Spans::from(vec![
        Span::styled("URL: ", Style::default().fg(Color::Blue)),
        Span::raw(url),
    ]);
    let block = Paragraph::new(text).block(
        Block::default()
            .title(Span::styled(
                "URL",
                Style::default().add_modifier(Modifier::BOLD),
            ))
            .borders(check_border(border, 0)),
    );
    f.render_widget(block, chunks[0]);
    let text2 = Spans::from(vec![
        Span::styled("Folder: ", Style::default().fg(Color::Blue)),
        Span::raw(folder),
    ]);
    let block = Paragraph::new(text2).block(
        Block::default()
            .title(Span::styled(
                "Folder",
                Style::default().add_modifier(Modifier::BOLD),
            ))
            .borders(check_border(border, 1)),
    );
    f.render_widget(block, chunks[1]);
}

fn check_border(app: i32, block: i32) -> Borders {
    if block == app {
        return Borders::ALL;
    }
    return Borders::NONE;
}
