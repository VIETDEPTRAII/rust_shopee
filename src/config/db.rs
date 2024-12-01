use sqlx::mysql::MySqlPool;
use anyhow::Result;

pub async fn connect_db() -> Result<MySqlPool> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = MySqlPool::connect(&database_url).await?;
    Ok(pool)
}
