use crate::state::AppState;
use ratatui::Frame;

use super::input::draw_input;
use super::layout::LayoutConfig;
use super::result::draw_result;
use super::status::draw_status;
use super::suggestions::draw_suggestions;

/// 绘制UI界面（主函数，负责布局和协调）
pub fn draw_ui(f: &mut Frame, app_state: &AppState) {
    // 根据状态创建布局配置
    let layout = LayoutConfig::from_state(f, app_state);

    // 绘制输入区域（总是显示）
    draw_input(f, layout.input_area(), app_state);

    // 绘制补全建议（有建议时显示）
    if let Some(area) = layout.suggestions_area() {
        draw_suggestions(f, area, app_state);
    }

    // 绘制查询结果（有结果时显示）
    if let Some(area) = layout.result_area() {
        draw_result(f, area, app_state);
    }

    // 绘制状态栏（总是显示）
    draw_status(f, layout.status_area(), app_state);
}
