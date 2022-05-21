use crossterm::style::Attribute::Bold;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Paragraph, List, ListItem, ListState};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders},
    Frame,
};
use crate::app::App;

struct Events {
    // `items` is the state managed by your application.
    items: Vec<String>,
    // `state` is the state that can be modified by the UI. It stores the index of the selected
    // item as well as the offset computed during the previous draw call (used to implement
    // natural scrolling).
    state: ListState
}

impl Events {
    fn new(items: Vec<String>) -> Events {
        Events {
            items,
            state: ListState::default(),
        }
    }

    pub fn _set_items(&mut self, items: Vec<String>) {
        self.items = items;
        // We reset the state as the associated items have changed. This effectively reset
        // the selection as well as the stored offset.
        self.state = ListState::default();
    }

    // Select the next item. This will not be reflected until the widget is drawn in the
    // `Terminal::draw` callback using `Frame::render_stateful_widget`.
    pub fn nth(&mut self, index:usize) {
        self.state.select(Some(index-1));
    }
}


pub fn ui1<B: Backend>(f: &mut Frame<B>, app:&App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(0)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(60)
            ]
            .as_ref(),
        )
        .split(f.size());
    
    let text = Spans::from(vec![
        Span::styled("URL: ", Style::default().fg(Color::Green)),
        Span::raw(&app.url),
    ]);
    
    let block = Paragraph::new(text).block(
        Block::default()
            .title(Span::styled(
                "URL",
                Style::default().add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL).border_style(check_border(app.border, 0)),
    );
    
    f.render_widget(block, chunks[0]);
    
    let text2 = Spans::from(vec![
        Span::styled("Folder: ", Style::default().fg(Color::Green)),
        Span::raw(&app.folder),
    ]);

    let block = Paragraph::new(text2).block(
        Block::default()
            .title(Span::styled(
                "Folder",
                Style::default().add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL).border_style(check_border(app.border, 1)),
    );
    
    f.render_widget(block, chunks[1]);

    let extensions = vec![String::from("3gp"), String::from("aac"), String::from("flv"), String::from("m4a"), String::from("mp3"), String::from("mp4"), String::from("ogg"), String::from("wav"), String::from("webm")];

    let mut overall = Vec::new();
    for n in extensions.clone() {
        overall.push(ListItem::new(n));
    }

    let block = List::new(overall)
        .block(
                Block::default()
                    .title(Span::styled(
                    "Extension",
                    Style::default().add_modifier(Modifier::BOLD),
                ))
                .borders(Borders::ALL).border_style(check_border(app.border, 2)),
                )
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");
    
    let mut overall_highlight = Events::new(extensions);
    overall_highlight.nth(app.format_status as usize);

    f.render_stateful_widget(block, chunks[2], &mut overall_highlight.state);
}

fn check_border(app: i32, block: i32) -> Style {
    if block == app {
        return Style::default()
            .fg(Color::Blue);
    }
    return Style::default()
        .fg(Color::White)
}
