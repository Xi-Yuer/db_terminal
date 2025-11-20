use crate::state::AppState;
use crossterm::event::{KeyCode, KeyModifiers};

/// 处理光标移动
pub fn handle_cursor_movement(key_code: KeyCode, modifiers: KeyModifiers, app_state: &mut AppState) {
    match (key_code, modifiers.contains(KeyModifiers::CONTROL)) {
        (KeyCode::Left, false) => {
            app_state.move_cursor_left();
        }
        (KeyCode::Right, false) => {
            app_state.move_cursor_right();
        }
        (KeyCode::Left, true) | (KeyCode::Home, _) => {
            app_state.move_cursor_home();
        }
        (KeyCode::Right, true) | (KeyCode::End, _) => {
            app_state.move_cursor_end();
        }
        _ => {}
    }
}

