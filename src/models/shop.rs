use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;
use crate::middlewares::auth::Claims;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Shop {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct SignupData {
    #[validate(length(min = 2, max = 50, message = "Name must be between 2 and 50 characters"))]
    pub name: String,
    
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
    
    #[validate(length(min = 8, max = 50, message = "Password must be between 8 and 50 characters"),
              custom = "is_valid_password")]
    pub password: String,
}

impl Shop {
    pub async fn find_by_id(id: i32, pool: &sqlx::MySqlPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Shop>("SELECT * FROM shop WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(signup_data: SignupData, pool: &sqlx::MySqlPool) -> Result<Self, sqlx::Error> {
        // Check if email already exists
        if let Some(_) = Self::find_by_email(&signup_data.email, pool).await? {
            return Err(sqlx::Error::Protocol("Email already exists".into()));
        }

        let hashed_password = hash(signup_data.password.as_bytes(), DEFAULT_COST)
            .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

        let mut tx = pool.begin().await?;

        // First, insert the shop
        sqlx::query(
            "INSERT INTO shop (name, email, password) VALUES (?, ?, ?)"
        )
        .bind(signup_data.name)
        .bind(signup_data.email)
        .bind(hashed_password)
        .execute(&mut *tx)
        .await?;

        // Then fetch the created shop using last_insert_id()
        let shop = sqlx::query_as::<_, Shop>(
            "SELECT * FROM shop WHERE id = LAST_INSERT_ID()"
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(shop)
    }

    pub async fn find_by_email(email: &str, pool: &sqlx::MySqlPool) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Shop>("SELECT * FROM shop WHERE email = ?")
            .bind(email)
            .fetch_optional(pool)
            .await
    }

    pub async fn login(credentials: LoginCredentials, pool: &sqlx::MySqlPool) -> Result<Option<String>, sqlx::Error> {
        let shop = Self::find_by_email(&credentials.email, pool).await?;

        if let Some(shop) = shop {
            if verify(credentials.password.as_bytes(), &shop.password)
                .map_err(|e| sqlx::Error::Protocol(e.to_string()))? 
            {
                let exp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .map_err(|e| sqlx::Error::Protocol(e.to_string()))?
                    .as_secs() as usize + 24 * 3600;

                let claims = Claims {
                    sub: shop.id,
                    exp,
                };

                let jwt_secret = std::env::var("JWT_SECRET")
                    .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret(jwt_secret.as_bytes()),
                )
                .map_err(|e| sqlx::Error::Protocol(e.to_string()))?;

                return Ok(Some(token));
            }
        }

        Ok(None)
    }
}

pub fn is_valid_password(password: &str) -> Result<(), validator::ValidationError> {
    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    
    if has_lowercase && has_uppercase && has_digit {
        Ok(())
    } else {
        Err(validator::ValidationError::new(
            "Password must contain at least one uppercase letter, one lowercase letter, and one number"
        ))
    }
}
