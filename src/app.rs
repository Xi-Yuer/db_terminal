use crate::db::Database;
use crate::event::{handle_key_event, Action};
use crate::state::AppState;
use crate::ui::draw_ui;
use crossterm::event::{self};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::io;

/// 应用主循环
pub async fn run(
    mut terminal: Terminal<CrosstermBackend<std::io::Stdout>>,
    db: Database, // 添加数据库参数
) -> io::Result<Terminal<CrosstermBackend<std::io::Stdout>>> {
    // 初始化应用状态
    let mut app_state = AppState::new();
    loop {
        // 绘制UI（传入状态）
        terminal.draw(|f| draw_ui(f, &app_state))?;

        // 处理事件
        if event::poll(std::time::Duration::from_millis(50))? {
            let evt = event::read()?;
            let action = handle_key_event(evt, &mut app_state, &db).await;
            match action {
                Action::Quit => break,
                Action::Continue => {}
            }
        }
    }

    Ok(terminal)
}
