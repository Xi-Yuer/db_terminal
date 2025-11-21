use crate::db::Database;
use crate::event::{Action, handle_key_event};
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
    let mut needs_redraw = true;
    let mut last_redraw = std::time::Instant::now();

    // 绘制UI（传入状态）
    terminal.draw(|f| draw_ui(f, &app_state))?;
    loop {
        // 检查是否需要渲染
        let should_redraw =
            needs_redraw || last_redraw.elapsed() > std::time::Duration::from_millis(60); // FPS
        if should_redraw {
            terminal.draw(|f| draw_ui(f, &app_state))?;
            needs_redraw = false;
            last_redraw = std::time::Instant::now();
        }
        // 非阻塞事件轮询
        if event::poll(std::time::Duration::from_millis(10))? {
            let evt = event::read()?;
            let action = handle_key_event(evt, &mut app_state, &db).await;

            needs_redraw = true; // 标记需要重绘

            match action {
                Action::Quit => break,
                Action::Continue => {}
            }
        }
    }

    Ok(terminal)
}
