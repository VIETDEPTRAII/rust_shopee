use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::Row;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Shop {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl Shop {
    pub async fn find_by_id(id: i32, pool: &sqlx::MySqlPool) -> Result<Option<Self>, sqlx::Error> {
        let shop = sqlx::query("SELECT * FROM shop WHERE id = ?")
            .bind(id)
            .map(|row: sqlx::mysql::MySqlRow| Shop {
                id: row.try_get("id").unwrap(),
                name: row.try_get("name").unwrap(),
                email: row.try_get("email").unwrap(),
                password: row.try_get("password").unwrap(),
            })
            .fetch_optional(pool)
            .await?;

        Ok(shop)
    }
}
