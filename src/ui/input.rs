use crate::state::AppState;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

/// 绘制输入区域（带光标）
pub fn draw_input(f: &mut Frame, area: Rect, app_state: &AppState) {
    let input_block = Block::default()
        .title("SQL 输入")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let input_text = if app_state.input.is_empty() {
        vec![Line::from(vec![Span::styled(
            "输入 SQL 查询... (Ctrl+C 或 ESC 退出)",
            Style::default().fg(Color::DarkGray),
        )])]
    } else {
        // 显示输入文本，并在光标位置插入光标符号
        let cursor_pos = app_state.cursor_position.min(app_state.input.len());
        let before_cursor = &app_state.input[..cursor_pos];
        let after_cursor = &app_state.input[cursor_pos..];

        let lines: Vec<Line> = vec![Line::from(vec![
            Span::styled(before_cursor, Style::default().fg(Color::White)),
            Span::styled("|", Style::default().fg(Color::Yellow).bg(Color::DarkGray)),
            Span::styled(after_cursor, Style::default().fg(Color::White)),
        ])];
        lines
    };

    let input_paragraph = Paragraph::new(input_text)
        .block(input_block)
        .wrap(Wrap { trim: true });

    f.render_widget(input_paragraph, area);
}
