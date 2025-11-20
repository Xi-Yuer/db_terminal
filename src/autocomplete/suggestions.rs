/// 补全建议
#[derive(Debug, Clone)]
pub struct Suggestion {
    pub text: String,
    pub description: String,
}

/// SQL 关键字列表
const SQL_KEYWORDS: &[&str] = &[
    "SELECT", "FROM", "WHERE", "INSERT", "UPDATE", "DELETE", "CREATE", "DROP", "ALTER", "TABLE",
    "DATABASE", "JOIN", "INNER", "LEFT", "RIGHT", "FULL", "OUTER", "ON", "AS", "AND", "OR", "NOT",
    "IN", "LIKE", "BETWEEN", "ORDER", "BY", "GROUP", "HAVING", "LIMIT", "OFFSET", "COUNT", "SUM",
    "AVG", "MAX", "MIN", "DISTINCT", "UNION", "ALL", "EXISTS", "CASE", "WHEN", "THEN", "ELSE",
    "END",
];

/// 获取补全建议
pub fn get_suggestions(input: &str, history: &[String], max_results: usize) -> Vec<Suggestion> {
    let mut suggestions = Vec::new();
    // 原始全量输入
    let input_full = input.to_string();
    // 获取到输入的最后一个单词，空格分割
    let input_last_word = input.split(" ").last().unwrap_or("");
    // 如果输入为空，返回常用关键字
    if input_last_word.trim().is_empty() {
        return SQL_KEYWORDS
            .iter()
            .take(max_results)
            .map(|&kw| Suggestion {
                text: kw.to_string(),
                description: format!("SQL 关键字: {}", kw),
            })
            .collect();
    }

    let input_first_word_upper = input_last_word.to_uppercase();

    // 1. 匹配 SQL 关键字
    for &keyword in SQL_KEYWORDS {
        if keyword.starts_with(&input_first_word_upper) {
            suggestions.push(Suggestion {
                text: keyword.to_string(),
                description: format!("SQL 关键字: {}", keyword),
            });
        }
    }

    // 2. 从历史记录中提取建议
    for hist in history.iter().rev() {
        if hist.to_uppercase().starts_with(&input_full.to_uppercase())
            && !suggestions
                .iter()
                .any(|s| s.text.to_uppercase() == hist.to_uppercase())
        {
            suggestions.push(Suggestion {
                text: hist.to_string(),
                description: format!("来自历史: {}", hist),
            });
        }
    }

    // 3. 限制结果数量并排序
    suggestions.truncate(max_results);
    suggestions.sort_by(|a, b| {
        // 优先显示完全匹配的
        let a_starts = a.text.to_uppercase().starts_with(&input_first_word_upper);
        let b_starts = b.text.to_uppercase().starts_with(&input_first_word_upper);
        match (a_starts, b_starts) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.text.len().cmp(&b.text.len()),
        }
    });

    suggestions
}
