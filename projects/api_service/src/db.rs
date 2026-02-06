use crate::error::AppError;
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;
use std::path::Path;

pub async fn init_pool(database_url: &str) -> Result<SqlitePool, AppError> {
    ensure_database_dir(database_url).await?;

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}

async fn ensure_database_dir(database_url: &str) -> Result<(), AppError> {
    if let Some(path) = database_url.strip_prefix("sqlite://") {
        let db_path = Path::new(path);
        if let Some(parent) = db_path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }
    }
    Ok(())
}
