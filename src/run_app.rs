use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use crate::ui::ui1;
use crate::app::App;
use tui::{
    backend::Backend,
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

pub fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App, tick_rate: Duration) -> io::Result<()> {
    let mut last_tick = Instant::now();
        loop {
            terminal.draw(|f| ui1(f, app));

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if crossterm::event::poll(timeout).expect("for some reason...") {
                if let Event::Key(key) = event::read().unwrap() {
                    match key.code {
                        KeyCode::Char(c) => app.on_key(c),
                        KeyCode::Backspace => app.on_backspace(),
                        KeyCode::Left => app.on_left(),
                        KeyCode::Up => app.on_up(),
                        KeyCode::Right => app.on_right(),
                        KeyCode::Down => app.on_down(),
                        KeyCode::Enter => app.on_enter(),
                        KeyCode::Esc => return Ok(()),
                        _ => {}
                    }
                }
            }
            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
    }     
}
