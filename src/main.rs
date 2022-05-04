mod ui;
mod app;
mod run_app;

use crossterm::terminal::enable_raw_mode;
use std::io;
use std::process::Command;
use tui::backend::CrosstermBackend;
use tui::{terminal, Terminal};
use ui::ui1;
use crate::app::App;
use run_app::run_app;

fn main() {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).unwrap();
    enable_raw_mode().unwrap();
    Command::new("clear").spawn().unwrap();
    let app = App::new();
    terminal.draw(|f| ui1(f, app)).unwrap();
}
