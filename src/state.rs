use crate::autocomplete::Suggestion;
use crate::db::QueryResult;

/// 应用状态
pub struct AppState {
    /// 用户输入的文本
    pub input: String,
    /// 光标位置（字符索引）
    pub cursor_position: usize,
    /// 状态
    pub status: String,
    /// 历史记录
    pub history: Vec<String>,
    /// 当前浏览历史的索引
    pub history_index: Option<usize>,
    /// 补全建议列表
    pub suggestions: Vec<Suggestion>,
    /// 当前选中的建议索引
    pub suggestion_index: Option<usize>,
    /// 查询结果
    pub query_result: Option<QueryResult>,
}

impl AppState {
    /// 创建新的应用状态
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor_position: 0,
            status: "等待输入...".to_string(),
            history: vec![],
            history_index: None,
            suggestions: Vec::new(),
            suggestion_index: None,
            query_result: None,
        }
    }

    /// 在光标位置插入字符
    pub fn insert_char(&mut self, c: char) {
        let pos = self.cursor_position.min(self.input.len());
        self.input.insert(pos, c);
        self.cursor_position = pos + 1;
    }

    /// 在光标位置删除字符（Backspace）
    pub fn delete_before_cursor(&mut self) {
        if self.cursor_position > 0 {
            let pos = self.cursor_position - 1;
            self.input.remove(pos);
            self.cursor_position = pos;
        }
    }

    /// 在光标位置删除字符（Delete）
    pub fn delete_after_cursor(&mut self) {
        if self.cursor_position < self.input.len() {
            self.input.remove(self.cursor_position);
        }
    }

    /// 移动光标向左
    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    /// 移动光标向右
    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input.len() {
            self.cursor_position += 1;
        }
    }

    /// 移动光标到行首
    pub fn move_cursor_home(&mut self) {
        self.cursor_position = 0;
    }

    /// 移动光标到行尾
    pub fn move_cursor_end(&mut self) {
        self.cursor_position = self.input.len();
    }

    /// 清空输入
    pub fn clear_input(&mut self) {
        self.input.clear();
        self.cursor_position = 0;
    }

    /// 设置状态
    pub fn set_status<S: Into<String>>(&mut self, msg: S) {
        self.status = msg.into();
    }

    /// 添加历史记录
    pub fn push_history(&mut self, sql: String) {
        if !sql.trim().is_empty() {
            self.history.push(sql);
        }
        self.history_index = None; // 重置历史浏览状态
    }

    /// 查找前一个记录
    pub fn history_prev(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let new_index = match self.history_index {
            Some(idx) if idx > 0 => idx - 1,
            Some(0) => 0,
            None => self.history.len() - 1,
            Some(_) => 0,
        };
        self.history_index = Some(new_index);
        self.input = self.history[new_index].clone();
        self.cursor_position = self.input.len(); // 光标移到末尾
    }

    /// 查找下一个记录
    pub fn history_next(&mut self) {
        if self.history.is_empty() {
            return;
        }
        match self.history_index {
            Some(idx) if idx + 1 < self.history.len() => {
                self.history_index = Some(idx + 1);
                self.input = self.history[idx + 1].clone();
                self.cursor_position = self.input.len(); // 光标移到末尾
            }
            _ => {
                self.history_index = None;
                self.input.clear();
                self.cursor_position = 0;
            }
        }
    }

    /// 更新补全建议
    pub fn update_suggestions(&mut self) {
        self.suggestions = crate::autocomplete::get_suggestions(&self.input, &self.history, 10);
        self.suggestion_index = None; // 重置选中索引
    }

    /// 选择下一个建议
    pub fn next_suggestion(&mut self) {
        if self.suggestions.is_empty() {
            return;
        }
        let new_index = match self.suggestion_index {
            Some(idx) if idx + 1 < self.suggestions.len() => idx + 1,
            Some(_) => 0, // 循环到第一个
            None => 0,
        };
        self.suggestion_index = Some(new_index);
    }

    /// 选择上一个建议
    pub fn prev_suggestion(&mut self) {
        if self.suggestions.is_empty() {
            return;
        }
        let new_index = match self.suggestion_index {
            Some(idx) if idx > 0 => idx - 1,
            Some(0) => self.suggestions.len() - 1, // 循环到最后一个
            None => self.suggestions.len() - 1,
            Some(_) => 0,
        };
        self.suggestion_index = Some(new_index);
    }

    /// 应用当前选中的建议
    pub fn apply_suggestion(&mut self) -> bool {
        if let Some(idx) = self.suggestion_index {
            if let Some(suggestion) = self.suggestions.get(idx) {
                // 找到输入中最后一个单词的位置
                let input_trimmed = self.input.trim_end();
                if let Some(last_space) = input_trimmed.rfind(' ') {
                    // 替换最后一个单词
                    self.input = format!(
                        "{} {} {}",
                        &input_trimmed[..=last_space].trim_end(),
                        suggestion.text,
                        " ",
                    );
                } else {
                    // 没有空格，直接替换整个输入，用在当输入只有一个单词的时候,替换全部的输入
                    self.input = format!("{} {}", suggestion.text, " ");
                }
                self.cursor_position = self.input.len(); // 光标移到末尾
                self.update_suggestions();
                return true;
            }
        }
        false
    }

    /// 设置查询结果
    pub fn set_query_result(&mut self, result: QueryResult) {
        self.query_result = Some(result);
    }

    /// 清空查询结果
    pub fn clear_query_result(&mut self) {
        self.query_result = None;
    }
}
