use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProductCategory {
    pub product_id: i64,
    pub category_id: i64,
    pub created_at: DateTime<Utc>,
}

impl ProductCategory {
    pub async fn find_categories_for_product(product_id: i64, pool: &sqlx::MySqlPool) -> Result<Vec<i64>, sqlx::Error> {
        sqlx::query_scalar::<_, i64>(
            "SELECT category_id FROM products_categories WHERE product_id = ?"
        )
        .bind(product_id)
        .fetch_all(pool)
        .await
    }

    pub async fn find_products_for_category(category_id: i64, pool: &sqlx::MySqlPool) -> Result<Vec<i64>, sqlx::Error> {
        sqlx::query_scalar::<_, i64>(
            "SELECT product_id FROM products_categories WHERE category_id = ?"
        )
        .bind(category_id)
        .fetch_all(pool)
        .await
    }

    pub async fn add_category_to_product(product_id: i64, category_id: i64, pool: &sqlx::MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO products_categories (product_id, category_id) VALUES (?, ?)"
        )
        .bind(product_id)
        .bind(category_id)
        .execute(pool)
        .await?;
        
        Ok(())
    }

    pub async fn remove_category_from_product(product_id: i64, category_id: i64, pool: &sqlx::MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "DELETE FROM products_categories WHERE product_id = ? AND category_id = ?"
        )
        .bind(product_id)
        .bind(category_id)
        .execute(pool)
        .await?;
        
        Ok(())
    }
} 