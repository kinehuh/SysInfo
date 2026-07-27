pub mod dashboard;
pub mod processes;
pub mod widgets;

use ratatui::Frame;
use crate::app::state::{AppState, AppMode};

pub fn draw(f: &mut Frame, state: &mut AppState) {
    match state.mode {
        AppMode::Dashboard => dashboard::draw(f, state),
        AppMode::Processes => processes::draw(f, state),
    }
}
