use crate::state::AppState;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

/// 绘制状态栏
pub fn draw_status(f: &mut Frame, area: Rect, app_state: &AppState) {
    let status_block = Block::default()
        .title("状态")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow));

    let status_text = vec![
        Line::from(vec![
            ratatui::text::Span::styled("提示: ", Style::default().fg(Color::Cyan)),
            ratatui::text::Span::styled("Ctrl+C 或 ESC 退出", Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            ratatui::text::Span::styled("输入长度: ", Style::default().fg(Color::Cyan)),
            ratatui::text::Span::styled(
                format!("{} 字符", app_state.input.len()),
                Style::default().fg(Color::Green),
            ),
        ]),
        Line::from(vec![
            ratatui::text::Span::styled("状态: ", Style::default().fg(Color::Cyan)),
            ratatui::text::Span::styled(
                format!("{}", app_state.status),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            ratatui::text::Span::styled("历史: ", Style::default().fg(Color::Cyan)),
            ratatui::text::Span::styled(
                match app_state.history_index {
                    Some(idx) => format!("{}/{}", idx + 1, app_state.history.len()),
                    None => format!("无 (共 {})", app_state.history.len()),
                },
                Style::default().fg(Color::Magenta),
            ),
        ]),
    ];

    let status_paragraph = Paragraph::new(status_text)
        .block(status_block)
        .wrap(Wrap { trim: true });

    f.render_widget(status_paragraph, area);
}

