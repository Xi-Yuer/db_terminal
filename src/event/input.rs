use crate::state::AppState;

/// 处理字符输入
pub fn handle_char_input(c: char, app_state: &mut AppState) {
    app_state.insert_char(c);
    app_state.update_suggestions();
}

/// 处理删除字符（Backspace）
pub fn handle_backspace(app_state: &mut AppState) {
    app_state.delete_before_cursor();
    app_state.update_suggestions();
}

/// 处理删除字符（Delete）
pub fn handle_delete(app_state: &mut AppState) {
    app_state.delete_after_cursor();
    app_state.update_suggestions();
}

