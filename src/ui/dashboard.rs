use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Row, Table, BorderType},
    Frame,
};
use crate::app::state::AppState;
use crate::system::{cpu, memory, network, disks};
use crate::ui::widgets::{standard_block, format_bytes};

pub fn draw(f: &mut Frame, state: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Percentage(30), // CPU / RAM
            Constraint::Percentage(30), // GPU / Disk
            Constraint::Percentage(20), // Network
            Constraint::Percentage(20), // System Info
        ])
        .split(f.area());

    // --- Header ---
    let header_text = Paragraph::new(Line::from(vec![
        Span::styled("SYS", Style::default().fg(Color::Cyan)),
        Span::styled("INFO", Style::default().fg(Color::Magenta)),
        Span::raw(" - Windows System Monitor (Press 'P' for Processes, 'Q' to Quit)"),
    ]))
    .block(Block::default().borders(Borders::ALL).border_type(BorderType::Double));
    f.render_widget(header_text, chunks[0]);

    // --- CPU & RAM ---
    let cpu_ram_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let cpu_info = cpu::get_cpu_info(&state.monitor.sys);
    let cpu_gauge = Gauge::default()
        .block(standard_block("CPU"))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(cpu_info.global_usage as u16)
        .label(format!("{} - {:.1}%", cpu_info.name, cpu_info.global_usage));
    f.render_widget(cpu_gauge, cpu_ram_chunks[0]);

    let mem_info = memory::get_memory_info(&state.monitor.sys);
    let mem_percent = if mem_info.total_ram > 0 {
        ((mem_info.used_ram as f64 / mem_info.total_ram as f64) * 100.0) as u16
    } else {
        0
    };
    
    let mem_gauge = Gauge::default()
        .block(standard_block("MEMORY"))
        .gauge_style(Style::default().fg(Color::Magenta))
        .percent(mem_percent)
        .label(format!("{} / {}", format_bytes(mem_info.used_ram), format_bytes(mem_info.total_ram)));
    f.render_widget(mem_gauge, cpu_ram_chunks[1]);

    // --- GPU & Storage ---
    let gpu_disk_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    if state.settings.show_gpu {
        if let Some(gpu) = &state.monitor.gpu.info {
            let gpu_usage = gpu.usage as u16;
            let mut label = format!("{} - {}%", gpu.name, gpu_usage);
            if state.settings.show_temperature && gpu.temperature > 0 {
                label.push_str(&format!(" ({}°C)", gpu.temperature));
            }
            if gpu.total_vram > 0 {
                label.push_str(&format!(" | VRAM: {}/{}", format_bytes(gpu.used_vram), format_bytes(gpu.total_vram)));
            }
            
            let gpu_gauge = Gauge::default()
                .block(standard_block("GPU"))
                .gauge_style(Style::default().fg(Color::Green))
                .percent(gpu_usage)
                .label(label);
            f.render_widget(gpu_gauge, gpu_disk_chunks[0]);
        } else {
            let gpu_missing = Paragraph::new("No supported GPU found.")
                .block(standard_block("GPU"));
            f.render_widget(gpu_missing, gpu_disk_chunks[0]);
        }
    } else {
        let gpu_disabled = Paragraph::new("GPU monitoring disabled in settings.")
            .block(standard_block("GPU"));
        f.render_widget(gpu_disabled, gpu_disk_chunks[0]);
    }

    let disks = disks::get_disks_info(&state.monitor.disks);
    let mut disk_lines = vec![];
    for d in disks.iter().take(3) { // Limit to 3 drives to avoid overflow
        let used = d.total_space.saturating_sub(d.available_space);
        let pct = if d.total_space > 0 {
            (used as f64 / d.total_space as f64) * 100.0
        } else {
            0.0
        };
        disk_lines.push(Line::from(format!("{}: {}/{} ({:.1}%) - [{}]", d.name, format_bytes(used), format_bytes(d.total_space), pct, d.file_system)));
    }
    let disks_p = Paragraph::new(disk_lines).block(standard_block("STORAGE"));
    f.render_widget(disks_p, gpu_disk_chunks[1]);

    // --- Network ---
    let nets = network::get_network_info(&state.monitor.networks);
    let mut net_lines = vec![];
    for n in nets.iter().take(3) {
        net_lines.push(Line::from(format!("{}: DL {}/s | UL {}/s", n.name, format_bytes(n.rx_bytes), format_bytes(n.tx_bytes))));
    }
    let nets_p = Paragraph::new(net_lines).block(standard_block("NETWORK"));
    f.render_widget(nets_p, chunks[3]);

    // --- System Info ---
    use sysinfo::System;
    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    let host_name = System::host_name().unwrap_or_else(|| "Unknown".to_string());
    let uptime = System::uptime(); // in seconds
    let hours = uptime / 3600;
    let mins = (uptime % 3600) / 60;
    
    let sys_lines = vec![
        Line::from(format!("OS: {} {}", os_name, os_version)),
        Line::from(format!("Hostname: {}", host_name)),
        Line::from(format!("Uptime: {}h {}m", hours, mins)),
        Line::from(format!("Total Processes: {}", state.monitor.sys.processes().len())),
    ];
    let sys_p = Paragraph::new(sys_lines).block(standard_block("SYSTEM"));
    f.render_widget(sys_p, chunks[4]);
}
