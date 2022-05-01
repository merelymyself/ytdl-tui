mod ui;

use crossterm::terminal::enable_raw_mode;
use std::io;
use std::process::Command;
use tui::backend::CrosstermBackend;
use tui::{terminal, Terminal};
use ui::ui1;

fn main() {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    enable_raw_mode().unwrap();
    Command::new("clear").spawn().unwrap();
    terminal.draw(|f| ui1(f, "test", 1, "test2"));
}
