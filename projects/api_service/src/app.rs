use crate::db::init_pool;
use crate::routes::build_router;
use axum::Router;

pub async fn build_app() -> Result<Router, crate::error::AppError> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| String::from("sqlite://data/api_service.db"));

    let pool = init_pool(&database_url).await?;
    Ok(build_router(pool))
}
