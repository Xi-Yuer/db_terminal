use crate::db::Database;
use crate::state::AppState;
use crossterm::event::{Event, KeyCode, KeyEventKind};

use super::cursor::handle_cursor_movement;
use super::input::{handle_backspace, handle_char_input, handle_delete};
use super::navigation::{handle_down, handle_tab, handle_up};
use super::quit::is_quit_key;
use super::sql::execute_sql;

/// 应用动作
#[derive(Debug, PartialEq)]
pub enum Action {
    Quit,     // 退出应用
    Continue, // 继续运行
}

/// 处理键盘事件（主分发器）
pub async fn handle_key_event(event: Event, app_state: &mut AppState, db: &Database) -> Action {
    if let Event::Key(key) = event {
        if key.kind != KeyEventKind::Press {
            return Action::Continue;
        }

        // 检查退出键
        if is_quit_key(key.code, key.modifiers) {
            return Action::Quit;
        }

        // 根据按键类型分发到不同的处理函数
        match key.code {
            KeyCode::Char(c) => {
                handle_char_input(c, app_state);
            }
            KeyCode::Backspace => {
                handle_backspace(app_state);
            }
            KeyCode::Delete => {
                handle_delete(app_state);
            }
            KeyCode::Left | KeyCode::Right | KeyCode::Home | KeyCode::End => {
                handle_cursor_movement(key.code, key.modifiers, app_state);
            }
            KeyCode::Up => {
                handle_up(app_state);
            }
            KeyCode::Down => {
                handle_down(app_state);
            }
            KeyCode::Enter => {
                execute_sql(app_state, db).await;
            }
            KeyCode::Tab => {
                handle_tab(app_state);
            }
            _ => {}
        }
    }

    Action::Continue
}
