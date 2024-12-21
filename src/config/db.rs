use sqlx::mysql::MySqlPool;
use std::{time::Duration, thread};

pub struct AppState {
    pub db: MySqlPool,
}

pub async fn connect_to_database(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
    let mut retries = 5;
    let mut last_error = None;

    while retries > 0 {
        match MySqlPool::connect(database_url).await {
            Ok(pool) => return Ok(pool),
            Err(e) => {
                println!("Failed to connect to database, retrying... ({} attempts left)", retries);
                last_error = Some(e);
                retries -= 1;
                thread::sleep(Duration::from_secs(2));
            }
        }
    }

    Err(last_error.unwrap())
}
