use crate::db::Database;
use crate::state::AppState;
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;

/// 执行 SQL 查询
pub async fn execute_sql(app_state: &mut AppState, db: &Database) {
    let sql = app_state.input.clone();
    
    if sql.trim().is_empty() {
        app_state.set_status("输入不能为空");
        return;
    }

    // 先解析 SQL
    let dialect = GenericDialect {};
    match Parser::parse_sql(&dialect, &sql) {
        Ok(_) => {
            // 解析成功，执行查询
            match db.execute_query(&sql).await {
                Ok(result) => {
                    app_state.set_status(format!("查询成功: {} 行", result.rows.len()));
                    app_state.set_query_result(result);
                }
                Err(err) => {
                    app_state.set_status(format!("查询失败: {}", err));
                    app_state.clear_query_result();
                }
            }
            // 保存历史
            app_state.push_history(sql);
        }
        Err(err) => {
            app_state.set_status(format!("解析失败: {}", err));
        }
    }

    // 清空输入并更新建议
    app_state.clear_input();
    app_state.update_suggestions();
}

