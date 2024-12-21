use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i64,
    pub shop_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProduct {
    pub shop_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub category_ids: Vec<i64>,
}

impl Product {
    pub async fn find_by_id(id: i64, pool: &sqlx::MySqlPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM products WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn find_by_shop_id(shop_id: i64, pool: &sqlx::MySqlPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>("SELECT * FROM products WHERE shop_id = ?")
            .bind(shop_id)
            .fetch_all(pool)
            .await
    }

    pub async fn create(data: CreateProduct, pool: &sqlx::MySqlPool) -> Result<Self, sqlx::Error> {
        let mut tx = pool.begin().await?;

        // Insert the product
        sqlx::query(
            "INSERT INTO products (shop_id, name, description, price) VALUES (?, ?, ?, ?)"
        )
        .bind(data.shop_id)
        .bind(&data.name)
        .bind(&data.description)
        .bind(data.price)
        .execute(&mut *tx)
        .await?;

        // Get the inserted product
        let product = sqlx::query_as::<_, Self>(
            "SELECT * FROM products WHERE id = LAST_INSERT_ID()"
        )
        .fetch_one(&mut *tx)
        .await?;

        // Insert category relationships
        for category_id in data.category_ids {
            sqlx::query(
                "INSERT INTO products_categories (product_id, category_id) VALUES (?, ?)"
            )
            .bind(product.id)
            .bind(category_id)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(product)
    }

    pub async fn find_by_category(category_id: i64, pool: &sqlx::MySqlPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT p.* FROM products p 
             INNER JOIN products_categories pc ON p.id = pc.product_id 
             WHERE pc.category_id = ?"
        )
        .bind(category_id)
        .fetch_all(pool)
        .await
    }
} 