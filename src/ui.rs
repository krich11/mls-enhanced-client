use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

/// Create a styled paragraph with border
pub fn create_bordered_paragraph<'a>(title: &'a str, content: &'a str, style: Style) -> Paragraph<'a> {
    Paragraph::new(content)
        .style(style)
        .block(Block::default().borders(Borders::ALL).title(title))
}

/// Create a colored status line
pub fn create_status_line(message: &str, is_error: bool) -> Line {
    let color = if is_error { Color::Red } else { Color::Green };
    Line::from(vec![
        Span::styled("[STATUS]", Style::default().fg(Color::Gray)),
        Span::raw(" "),
        Span::styled(message, Style::default().fg(color)),
    ])
}

/// Create a timestamp span
pub fn create_timestamp_span(timestamp: &str) -> Span {
    Span::styled(
        format!("[{}]", timestamp),
        Style::default().fg(Color::Gray),
    )
}

/// Create a username span
pub fn create_username_span(username: &str) -> Span {
    Span::styled(
        format!("{}: ", username),
        Style::default().fg(Color::Blue),
    )
}

/// Truncate text to fit within specified width
pub fn truncate_text(text: &str, max_width: usize) -> String {
    if text.len() <= max_width {
        text.to_string()
    } else {
        format!("{}...", &text[..max_width.saturating_sub(3)])
    }
}

/// Format file size in human readable format
pub fn format_file_size(size: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = size as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{:.0} {}", size, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}