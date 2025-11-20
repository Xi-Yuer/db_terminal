use crossterm::event::{KeyCode, KeyModifiers};

/// 检查是否是退出按键
pub fn is_quit_key(key_code: KeyCode, modifiers: KeyModifiers) -> bool {
    matches!(
        (key_code, modifiers.contains(KeyModifiers::CONTROL)),
        (KeyCode::Esc, _) | (KeyCode::Char('c'), true)
    )
}

