use actix_web::{get, web, HttpResponse, Responder};
use crate::AppState;
use crate::models::shop::Shop;

#[get("/shops/{id}")]
pub async fn get_shop_info(
    path: web::Path<i32>,
    data: web::Data<AppState>,
) -> impl Responder {
    let shop_id = path.into_inner();
    
    match Shop::find_by_id(shop_id, &data.db).await {
        Ok(Some(shop)) => HttpResponse::Ok().json(shop),
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "message": "Shop not found"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "message": "Internal server error"
        })),
    }
}