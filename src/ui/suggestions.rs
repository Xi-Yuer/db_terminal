use crate::state::AppState;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

/// 绘制补全建议列表
/// 注意：此函数只在有建议时被调用（由布局管理器控制）
pub fn draw_suggestions(f: &mut Frame, area: Rect, app_state: &AppState) {
    let suggestions_block = Block::default()
        .title("补全建议 (Tab 应用, ↑↓ 选择)")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Blue));

    let suggestion_lines: Vec<Line> = app_state
        .suggestions
        .iter()
        .enumerate()
        .map(|(idx, sug)| {
            let is_selected = app_state.suggestion_index == Some(idx);
            let style = if is_selected {
                Style::default().fg(Color::Yellow).add_modifier(
                    ratatui::style::Modifier::REVERSED | ratatui::style::Modifier::BOLD,
                )
            } else {
                Style::default().fg(Color::White)
            };
            Line::from(vec![
                ratatui::text::Span::styled(
                    format!("  {}  ", if is_selected { ">" } else { " " }),
                    style,
                ),
                ratatui::text::Span::styled(&sug.text, style),
                ratatui::text::Span::styled(
                    format!("  - {}", sug.description),
                    Style::default().fg(Color::DarkGray),
                ),
            ])
        })
        .collect();

    let suggestions_paragraph = Paragraph::new(suggestion_lines)
        .block(suggestions_block)
        .wrap(Wrap { trim: true });

    f.render_widget(suggestions_paragraph, area);
}
