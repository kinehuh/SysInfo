mod app;
mod config;
mod system;
mod ui;

use anyhow::Result;
use app::events::handle_events;
use app::state::AppState;
use config::settings::Settings;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, stdout};
use std::time::Duration;

fn main() -> Result<()> {
    // Load config
    let settings = Settings::load().unwrap_or_default();

    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // App state
    let mut state = AppState::new(settings);
    let tick_rate = Duration::from_millis(state.settings.refresh_rate);

    // Main loop
    while !state.should_quit {
        terminal.draw(|f| {
            ui::draw(f, &mut state);
        })?;

        handle_events(&mut state, tick_rate)?;
    }

    // Restore terminal
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
