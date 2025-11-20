use crate::state::AppState;

/// 处理向上导航
pub fn handle_up(app_state: &mut AppState) {
    if !app_state.suggestions.is_empty() {
        app_state.prev_suggestion();
    } else {
        app_state.history_prev();
    }
}

/// 处理向下导航
pub fn handle_down(app_state: &mut AppState) {
    if !app_state.suggestions.is_empty() {
        app_state.next_suggestion();
    } else {
        app_state.history_next();
    }
}

/// 处理 Tab 键（自动补全）
pub fn handle_tab(app_state: &mut AppState) {
    if app_state.suggestion_index.is_none() && !app_state.suggestions.is_empty() {
        // 第一次按 Tab，选中第一个建议
        app_state.suggestion_index = Some(0);
    } else {
        // 应用当前选中的建议
        app_state.apply_suggestion();
    }
}

