mod models;
mod controllers;
mod config;

use actix_web::{web, App, HttpServer, middleware::Logger};
use sqlx::mysql::MySqlPool;
use env_logger::Env;
use std::{time::Duration, thread};

pub struct AppState {
    db: MySqlPool,
}

async fn connect_to_database(database_url: &str) -> Result<MySqlPool, sqlx::Error> {
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    println!("Connecting to database...");
    let pool = connect_to_database(&database_url)
        .await
        .expect("Failed to connect to database");
    println!("Database connected successfully!");

    println!("Starting server at 0.0.0.0:8080");
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                db: pool.clone(),
            }))
            .service(
                web::scope("/api")
                    .service(controllers::shop::get_shop_info)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
