use crate::config::settings::Settings;
use crate::system::SystemMonitor;

#[derive(PartialEq)]
pub enum AppMode {
    Dashboard,
    Processes,
}

pub struct AppState {
    pub should_quit: bool,
    pub mode: AppMode,
    pub settings: Settings,
    pub monitor: SystemMonitor,
    // Process table state
    pub process_selected: usize,
    pub process_scroll: usize,
}

impl AppState {
    pub fn new(settings: Settings) -> Self {
        Self {
            should_quit: false,
            mode: AppMode::Dashboard,
            settings,
            monitor: SystemMonitor::new(),
            process_selected: 0,
            process_scroll: 0,
        }
    }

    pub fn on_tick(&mut self) {
        self.monitor.refresh();
    }

    pub fn toggle_mode(&mut self) {
        if self.mode == AppMode::Dashboard {
            self.mode = AppMode::Processes;
        } else {
            self.mode = AppMode::Dashboard;
        }
    }

    pub fn process_up(&mut self) {
        if self.process_selected > 0 {
            self.process_selected -= 1;
        }
    }

    pub fn process_down(&mut self, max: usize) {
        if self.process_selected < max.saturating_sub(1) {
            self.process_selected += 1;
        }
    }
}
