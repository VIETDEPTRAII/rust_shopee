use actix_web::{get, post, web, HttpResponse, Responder, HttpRequest, HttpMessage};
use crate::AppState;
use crate::models::category::{Category, CreateCategory, CreateCategoryRequest};
use crate::middlewares::auth::Claims;
use validator::Validate;

#[post("/categories")]
pub async fn create_category(
    req: HttpRequest,
    data: web::Json<CreateCategoryRequest>,
    state: web::Data<AppState>,
) -> impl Responder {
    if let Err(errors) = data.validate() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "message": "Validation failed",
            "errors": errors
        }));
    }

    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().unwrap();
    
    let category_data = CreateCategory {
        shop_id: claims.sub as i64,
        name: data.name.clone(),
    };

    match Category::create(category_data, &state.db).await {
        Ok(category) => HttpResponse::Created().json(category),
        Err(e) => {
            eprintln!("Error creating category: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Failed to create category"
            }))
        }
    }
}

#[get("/categories/{id}")]
pub async fn get_category(
    req: HttpRequest,
    path: web::Path<i64>,
    state: web::Data<AppState>,
) -> impl Responder {
    let category_id = path.into_inner();
    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().unwrap();
    
    match Category::find_by_id(category_id, &state.db).await {
        Ok(Some(category)) => {
            if category.shop_id == claims.sub as i64 {
                HttpResponse::Ok().json(category)
            } else {
                HttpResponse::NotFound().json(serde_json::json!({
                    "message": "Category not found"
                }))
            }
        },
        Ok(None) => HttpResponse::NotFound().json(serde_json::json!({
            "message": "Category not found"
        })),
        Err(e) => {
            eprintln!("Error fetching category: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Failed to fetch category"
            }))
        }
    }
}

#[get("/categories")]
pub async fn list_categories(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> impl Responder {
    let extensions = req.extensions();
    let claims = extensions.get::<Claims>().unwrap();
    
    match Category::find_by_shop_id(claims.sub as i64, &state.db).await {
        Ok(categories) => HttpResponse::Ok().json(categories),
        Err(e) => {
            eprintln!("Error listing categories: {:?}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "message": "Failed to list categories"
            }))
        }
    }
}
