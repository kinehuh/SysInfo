use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Row, Table, TableState, Paragraph, BorderType},
    Frame,
};
use crate::app::state::AppState;
use crate::ui::widgets::{standard_block, format_bytes};

pub fn draw(f: &mut Frame, state: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(5),    // Table
        ])
        .split(f.area());

    // --- Header ---
    let header_text = Paragraph::new(Line::from(vec![
        Span::styled("SYS", Style::default().fg(Color::Cyan)),
        Span::styled("INFO", Style::default().fg(Color::Magenta)),
        Span::raw(" - Processes (Press 'P' to return to Dashboard, 'Q' to Quit)"),
    ]))
    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Double));
    f.render_widget(header_text, chunks[0]);

    // --- Process Table ---
    let mut processes: Vec<_> = state.monitor.sys.processes().values().collect();
    // Sort by CPU usage descending
    processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));

    // Update state to respect bounds
    if state.process_selected >= processes.len() && !processes.is_empty() {
        state.process_selected = processes.len() - 1;
    }

    let rows: Vec<Row> = processes.iter().map(|p| {
        let pid = p.pid().as_u32().to_string();
        let name = p.name().to_string_lossy().to_string();
        let cpu = format!("{:.1}%", p.cpu_usage());
        let mem = format_bytes(p.memory());
        
        Row::new(vec![pid, name, cpu, mem])
    }).collect();

    let widths = [
        Constraint::Percentage(15), // PID
        Constraint::Percentage(45), // Name
        Constraint::Percentage(20), // CPU
        Constraint::Percentage(20), // Mem
    ];

    let table = Table::new(rows, widths)
        .header(Row::new(vec!["PID", "Name", "CPU", "Memory"])
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .bottom_margin(1))
        .block(standard_block("PROCESSES"))
        .row_highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
        .highlight_symbol(">> ");

    let mut table_state = TableState::default();
    table_state.select(Some(state.process_selected));

    f.render_stateful_widget(table, chunks[1], &mut table_state);
}
