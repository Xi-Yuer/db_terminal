use crate::state::AppState;
use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// UI 组件区域索引
#[derive(Debug, Clone, Copy)]
pub struct LayoutAreas {
    pub input: usize,
    pub suggestions: Option<usize>,
    pub result: Option<usize>,
    pub status: usize,
}

/// 布局配置
pub struct LayoutConfig {
    pub areas: LayoutAreas,
    pub chunks: Vec<Rect>,
}

impl LayoutConfig {
    /// 根据应用状态创建布局配置
    pub fn from_state(f: &ratatui::Frame, app_state: &AppState) -> Self {
        let has_suggestions = !app_state.suggestions.is_empty();
        let has_result = app_state.query_result.is_some();

        // 构建约束列表
        let mut constraints = vec![];
        let mut areas = LayoutAreas {
            input: 0,
            suggestions: None,
            result: None,
            status: 0,
        };

        let mut current_index = 0;

        // 输入区域（总是显示）
        constraints.push(Constraint::Length(6));
        areas.input = current_index;
        current_index += 1;

        // 补全建议区域（有建议时显示）
        if has_suggestions {
            constraints.push(Constraint::Length(6));
            areas.suggestions = Some(current_index);
            current_index += 1;
        }

        // 查询结果区域（有结果时显示）
        if has_result {
            if let Some(result) = &app_state.query_result {
                // 根据结果行数动态调整高度，但限制最大高度
                let height = (result.rows.len() + 5).min(30) as u16;
                constraints.push(Constraint::Length(height));
            } else {
                constraints.push(Constraint::Min(10));
            }
            areas.result = Some(current_index);
            current_index += 1;
        }

        // 状态栏（总是显示）
        constraints.push(Constraint::Length(6));
        areas.status = current_index;

        // 分割屏幕
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(f.size())
            .to_vec();

        Self { areas, chunks }
    }

    /// 获取输入区域
    pub fn input_area(&self) -> Rect {
        self.chunks[self.areas.input]
    }

    /// 获取补全建议区域
    pub fn suggestions_area(&self) -> Option<Rect> {
        self.areas.suggestions.map(|idx| self.chunks[idx])
    }

    /// 获取查询结果区域
    pub fn result_area(&self) -> Option<Rect> {
        self.areas.result.map(|idx| self.chunks[idx])
    }

    /// 获取状态栏区域
    pub fn status_area(&self) -> Rect {
        self.chunks[self.areas.status]
    }
}
