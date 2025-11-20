use sqlx::{Column, MySqlPool, Row, mysql::MySqlPoolOptions};

/// 数据库连接
pub struct Database {
    pool: MySqlPool,
}

impl Database {
    /// 连接到 MySQL 数据库
    pub async fn connect(url: &str) -> sqlx::Result<Self> {
        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(url)
            .await?;
        Ok(Self { pool })
    }

    /// 执行查询并返回结果
    pub async fn execute_query(&self, sql: &str) -> sqlx::Result<QueryResult> {
        let rows = sqlx::query(sql).fetch_all(&self.pool).await?;

        let mut columns = Vec::new();
        let mut data = Vec::new();

        // 处理第一行，获取列名
        if let Some(first_row) = rows.first() {
            columns = first_row
                .columns()
                .iter()
                .map(|col| col.name().to_string())
                .collect();
        }

        // 处理所有行数据
        for row in rows {
            let mut values = Vec::new();
            for i in 0..row.len() {
                // 尝试获取各种类型的值并转换为字符串
                let value = if let Ok(v) = row.try_get::<String, _>(i) {
                    v
                } else if let Ok(v) = row.try_get::<i64, _>(i) {
                    v.to_string()
                } else if let Ok(v) = row.try_get::<f64, _>(i) {
                    v.to_string()
                } else if let Ok(v) = row.try_get::<bool, _>(i) {
                    v.to_string()
                } else if row.try_get::<Option<String>, _>(i).is_ok() {
                    if let Ok(Some(v)) = row.try_get::<Option<String>, _>(i) {
                        v
                    } else {
                        "NULL".to_string()
                    }
                } else {
                    "NULL".to_string()
                };
                values.push(value);
            }
            data.push(values);
        }

        Ok(QueryResult {
            columns,
            rows: data,
        })
    }
}

/// 查询结果
pub struct QueryResult {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<String>>,
}
