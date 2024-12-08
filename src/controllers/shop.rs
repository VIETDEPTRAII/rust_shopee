use actix_web::{get, post, web, HttpResponse, Responder, HttpRequest, HttpMessage};
use crate::AppState;
use crate::models::shop::Shop;
use crate::models::shop::{LoginCredentials, SignupData};
use validator::Validate;
use crate::middlewares::auth::Claims;

#[get("/shops/me")]
pub async fn get_current_shop(
    req: HttpRequest,
    data: web::Data<AppState>,
) -> impl Responder {
    // Get claims from request extensions (set by AuthMiddleware)
    if let Some(claims) = req.extensions_mut().get::<Claims>() {
        match Shop::find_by_id(claims.sub, &data.db).await {
            Ok(Some(shop)) => HttpResponse::Ok().json(shop),
            Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
                "message": "Shop not found"
            })),
            Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Internal server error"
            })),
        }
    } else {
        HttpResponse::Unauthorized().json(serde_json::json!({
            "message": "Invalid token"
        }))
    }
}

#[post("/shops/signup")]
pub async fn signup(
    signup_data: web::Json<SignupData>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Validate input data
    if let Err(errors) = signup_data.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "errors": errors
        }));
    }

    match Shop::create(signup_data.into_inner(), &data.db).await {
        Ok(shop) => HttpResponse::Created().json(shop),
        Err(sqlx::Error::Protocol(msg)) => HttpResponse::BadRequest().json(serde_json::json!({
            "message": msg
        })),
        Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {
            HttpResponse::BadRequest().json(serde_json::json!({
                "message": "Email already exists"
            }))
        }
        Err(e) => {
            eprintln!("Error creating shop: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Failed to create shop"
            }))
        }
    }
}

#[post("/shops/login")]
pub async fn login(
    credentials: web::Json<LoginCredentials>,
    data: web::Data<AppState>,
) -> impl Responder {
    match Shop::login(credentials.into_inner(), &data.db).await {
        Ok(Some(token)) => HttpResponse::Ok().json(serde_json::json!({
            "token": token
        })),
        Ok(None) => HttpResponse::Unauthorized().json(serde_json::json!({
            "message": "Invalid credentials"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "message": "Internal server error"
        })),
    }
}
