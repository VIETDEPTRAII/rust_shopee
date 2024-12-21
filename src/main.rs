mod models;
mod controllers;
mod config;
mod middlewares;
use crate::middlewares::auth::AuthMiddleware;
use crate::config::db::{AppState, connect_to_database};

use actix_web::{web, App, HttpServer, middleware::Logger};
use env_logger::Env;

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
                    .service(controllers::shop::signup)
                    .service(controllers::shop::login)
                    .service(
                        web::scope("")
                            .wrap(AuthMiddleware)
                            .service(controllers::shop::get_current_shop)
                            .service(controllers::category::create_category)
                            .service(controllers::category::get_category)
                            .service(controllers::category::list_categories)
                    )
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
