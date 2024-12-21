use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i64,
    pub shop_id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCategory {
    pub shop_id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateCategoryRequest {
    #[validate(length(min = 1, max = 50, message = "Category name must be between 1 and 50 characters"))]
    #[validate(custom = "validate_category_name")]
    pub name: String,
}

fn validate_category_name(name: &str) -> Result<(), validator::ValidationError> {
    if name.trim().is_empty() {
        return Err(validator::ValidationError::new("Category name cannot be empty"));
    }
    Ok(())
}

impl Category {
    pub async fn find_by_id(id: i64, pool: &sqlx::MySqlPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM categories WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_shop_id(shop_id: i64, pool: &sqlx::MySqlPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM categories WHERE shop_id = ?")
            .bind(shop_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(data: CreateCategory, pool: &sqlx::MySqlPool) -> Result<Self, sqlx::Error> {
        let mut tx = pool.begin().await?;

        sqlx::query(
            "INSERT INTO categories (shop_id, name) VALUES (?, ?)"
        )
        .bind(data.shop_id)
        .bind(data.name)
        .execute(&mut *tx)
        .await?;

        let category = sqlx::query_as::<_, Self>(
            "SELECT * FROM categories WHERE id = LAST_INSERT_ID()"
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(category)
    }
}
