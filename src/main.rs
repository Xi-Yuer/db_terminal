mod app;
mod autocomplete;
mod db;
mod event;
mod state;
mod terminal;
mod ui;

use terminal::{init, restore};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化终端
    let terminal = init()?;

    // 连接数据库（请修改为你的 MySQL 连接字符串）
    let database_url = "mysql://root:2214380963Wx!!@localhost:3306/test";
    let db = match db::Database::connect(database_url).await {
        Ok(db) => db,
        Err(e) => {
            eprintln!("数据库连接失败: {}", e);
            eprintln!("请检查连接字符串: {}", database_url);
            return Err(anyhow::anyhow!("数据库连接失败: {}", e));
        }
    };

    // 运行应用
    let terminal = app::run(terminal, db).await?;

    // 恢复终端
    restore(terminal)?;

    Ok(())
}
