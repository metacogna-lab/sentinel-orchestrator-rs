// UI components for the TUI

use crate::types::*;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

/// Render the main menu
pub fn render_main_menu(f: &mut Frame, selected: usize) {
    let menu_items = vec![
        "Chat Mode",
        "Investigation Mode",
        "Debugging Mode",
        "System Status",
        "Quit",
    ];

    let items_len = menu_items.len();
    let items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Span::styled(*item, style))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Sentinel Orchestrator CLI")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    let area = centered_rect(40, items_len as u16 + 2, f.size());
    f.render_widget(list, area);
}

/// Render chat interface
pub fn render_chat(f: &mut Frame, messages: &[CanonicalMessage], input: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(f.size());

    // Messages area
    let message_items: Vec<ListItem> = messages
        .iter()
        .map(|msg| {
            let role_color = match msg.role {
                Role::User => Color::Cyan,
                Role::Assistant => Color::Green,
                Role::System => Color::Yellow,
            };

            let role_text = match msg.role {
                Role::User => "User",
                Role::Assistant => "Assistant",
                Role::System => "System",
            };

            let timestamp = msg.timestamp.format("%H:%M:%S").to_string();
            let header = format!("[{}] {}", role_text, timestamp);
            let content = msg.content.clone();

            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(header, Style::default().fg(role_color).add_modifier(Modifier::BOLD)),
                ]),
                Line::from(content),
            ])
        })
        .collect();

    let messages_list = List::new(message_items)
        .block(
            Block::default()
                .title("Chat")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        );

    f.render_widget(messages_list, chunks[0]);

    // Input area
    let input_paragraph = Paragraph::new(input)
        .block(
            Block::default()
                .title("Input (Enter to send, Esc to cancel)")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(input_paragraph, chunks[1]);
}

/// Render system status
pub fn render_system_status(f: &mut Frame, health: &Option<HealthStatus>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(f.size());

    // Health status header
    let status_text = if let Some(health) = health {
        let status_color = match health.status {
            HealthState::Healthy | HealthState::Ready => Color::Green,
            HealthState::Alive => Color::Yellow,
            HealthState::Unhealthy => Color::Red,
        };

        let status_str = format!("{:?}", health.status);
        let timestamp = health.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string();

        vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::White)),
                Span::styled(status_str, Style::default().fg(status_color).add_modifier(Modifier::BOLD)),
                Span::raw(" | "),
                Span::styled("Last Check: ", Style::default().fg(Color::White)),
                Span::styled(timestamp, Style::default().fg(Color::Cyan)),
            ]),
        ]
    } else {
        vec![Line::from(vec![Span::styled(
            "Status: Not checked",
            Style::default().fg(Color::Yellow),
        )])]
    };

    let status_block = Paragraph::new(status_text)
        .block(
            Block::default()
                .title("System Health")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Left);

    f.render_widget(status_block, chunks[0]);

    // Additional info area
    let info_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Endpoints:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  • /health - Health check"),
        Line::from("  • /health/ready - Readiness check"),
        Line::from("  • /health/live - Liveness check"),
        Line::from("  • /v1/chat/completions - Chat API"),
        Line::from(""),
        Line::from(vec![
            Span::styled("Navigation:", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
        ]),
        Line::from("  • Tab - Switch between modes"),
        Line::from("  • ↑/↓ - Navigate menu"),
        Line::from("  • Enter - Select"),
        Line::from("  • Esc - Go back / Cancel"),
        Line::from("  • q - Quit"),
    ];

    let info_block = Paragraph::new(info_text)
        .block(
            Block::default()
                .title("Information")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(info_block, chunks[1]);
}

/// Render investigation mode
pub fn render_investigation(f: &mut Frame, query: &str, results: &[String]) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(f.size());

    // Query input
    let query_paragraph = Paragraph::new(query)
        .block(
            Block::default()
                .title("Investigation Query")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(query_paragraph, chunks[0]);

    // Results
    let result_items: Vec<ListItem> = results
        .iter()
        .map(|result| ListItem::new(result.as_str()))
        .collect();

    let results_list = List::new(result_items)
        .block(
            Block::default()
                .title("Investigation Results")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta)),
        );

    f.render_widget(results_list, chunks[1]);
}

/// Render debugging mode
pub fn render_debugging(f: &mut Frame, logs: &[String]) {
    let log_items: Vec<ListItem> = logs
        .iter()
        .map(|log| {
            let style = if log.contains("ERROR") {
                Style::default().fg(Color::Red)
            } else if log.contains("WARN") {
                Style::default().fg(Color::Yellow)
            } else if log.contains("INFO") {
                Style::default().fg(Color::Cyan)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Span::styled(log.as_str(), style))
        })
        .collect();

    let logs_list = List::new(log_items)
        .block(
            Block::default()
                .title("Debug Logs")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red)),
        );

    f.render_widget(logs_list, f.size());
}

/// Render error message
pub fn render_error(f: &mut Frame, error: &str) {
    let error_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Error:", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(error),
        Line::from(""),
        Line::from(vec![
            Span::styled("Press Esc to dismiss", Style::default().fg(Color::Yellow)),
        ]),
    ];

    let error_block = Paragraph::new(error_text)
        .block(
            Block::default()
                .title("Error")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red)),
        )
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    let area = centered_rect(60, 8, f.size());
    f.render_widget(error_block, area);
}

/// Helper to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

