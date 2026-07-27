use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;
use anyhow::Result;
use crate::app::state::{AppState, AppMode};

pub fn handle_events(state: &mut AppState, tick_rate: Duration) -> Result<()> {
    if event::poll(tick_rate)? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Char('Q') => state.should_quit = true,
                    KeyCode::Char('p') | KeyCode::Char('P') => state.toggle_mode(),
                    KeyCode::Up => {
                        if state.mode == AppMode::Processes {
                            state.process_up();
                        }
                    }
                    KeyCode::Down => {
                        if state.mode == AppMode::Processes {
                            // TODO: dynamically calculate max process index from the UI layer
                            state.process_down(10000); 
                        }
                    }
                    _ => {}
                }
            }
        }
    } else {
        state.on_tick();
    }
    Ok(())
}
