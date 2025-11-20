use crate::state::AppState;
use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
    Frame,
};

/// 绘制查询结果表格
/// 注意：此函数只在有结果时被调用（由布局管理器控制）
pub fn draw_result(f: &mut Frame, area: Rect, app_state: &AppState) {
    // 布局管理器确保只在有结果时调用，但为了防御性编程，保留检查
    let result = match &app_state.query_result {
        Some(r) => r,
        None => {
            // 不应该到达这里，但为了安全起见
            let empty_block = Block::default()
                .title("查询结果 (等待查询...)")
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(empty_block, area);
            return;
        }
    };

    // 创建表头
    let header_cells: Vec<Cell> = result
        .columns
        .iter()
        .map(|c| {
            Cell::from(c.clone())
                .style(Style::default().fg(Color::Yellow).add_modifier(ratatui::style::Modifier::BOLD))
        })
        .collect();
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::DarkGray))
        .height(1);

    // 创建数据行
    let rows: Vec<Row> = result
        .rows
        .iter()
        .map(|row| {
            Row::new(
                row.iter()
                    .map(|cell| Cell::from(cell.clone()))
                    .collect::<Vec<_>>(),
            )
            .height(1)
        })
        .collect();

    // 计算列宽（简单实现：每列固定宽度）
    let widths: Vec<Constraint> = result
        .columns
        .iter()
        .map(|_| Constraint::Length(20))
        .collect();

    let table = Table::new(rows, widths)
        .header(header)
        .block(
            Block::default()
                .title(format!("查询结果 ({} 行)", result.rows.len()))
                .borders(Borders::ALL),
        )
        .column_spacing(1);

    f.render_widget(table, area);
}

