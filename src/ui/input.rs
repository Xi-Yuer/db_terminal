use crate::state::AppState;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
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

        // 将文本按行分割，处理多行情况
        let lines: Vec<Line> = if before_cursor.contains('\n') || after_cursor.contains('\n') {
            // 多行处理：找到光标所在的行
            let all_lines: Vec<&str> = app_state.input.lines().collect();
            let mut result = Vec::new();
            let mut char_count = 0;
            let mut cursor_line = 0;
            let mut cursor_col = 0;

            for (line_idx, line) in all_lines.iter().enumerate() {
                let line_len = line.len();
                if char_count + line_len >= cursor_pos {
                    cursor_line = line_idx;
                    cursor_col = cursor_pos - char_count;
                    break;
                }
                char_count += line_len + 1; // +1 for newline
            }

            for (line_idx, line) in all_lines.iter().enumerate() {
                if line_idx == cursor_line {
                    // 光标所在行：在光标位置插入光标
                    let before = &line[..cursor_col.min(line.len())];
                    let after = &line[cursor_col.min(line.len())..];
                    result.push(Line::from(vec![
                        Span::styled(before, Style::default().fg(Color::White)),
                        Span::styled("█", Style::default().fg(Color::Yellow).bg(Color::DarkGray)),
                        Span::styled(after, Style::default().fg(Color::White)),
                    ]));
                } else {
                    result.push(Line::from(vec![Span::styled(
                        *line,
                        Style::default().fg(Color::White),
                    )]));
                }
            }
            result
        } else {
            // 单行处理：简单情况
            vec![Line::from(vec![
                Span::styled(before_cursor, Style::default().fg(Color::White)),
                Span::styled("█", Style::default().fg(Color::Yellow).bg(Color::DarkGray)),
                Span::styled(after_cursor, Style::default().fg(Color::White)),
            ])]
        };

        lines
    };

    let input_paragraph = Paragraph::new(input_text)
        .block(input_block)
        .wrap(Wrap { trim: true });

    f.render_widget(input_paragraph, area);
}

