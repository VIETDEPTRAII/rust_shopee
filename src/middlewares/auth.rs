use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // shop_id
    pub exp: usize,
}

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        
        if let Some(auth_header) = auth_header {
            if let Ok(auth_str) = auth_header.to_str() {
                if auth_str.starts_with("Bearer ") {
                    let token = auth_str.trim_start_matches("Bearer ");
                    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
                    
                    match decode::<Claims>(
                        token,
                        &DecodingKey::from_secret(jwt_secret.as_bytes()),
                        &Validation::default(),
                    ) {
                        Ok(token_data) => {
                            req.extensions_mut().insert(token_data.claims);
                            let fut = self.service.call(req);
                            return Box::pin(async move {
                                let res = fut.await?;
                                Ok(res)
                            });
                        }
                        Err(_) => {
                            return Box::pin(async move {
                                Err(actix_web::error::InternalError::from_response(
                                    "",
                                    HttpResponse::Unauthorized()
                                        .json(serde_json::json!({
                                            "message": "Invalid token"
                                        }))
                                ).into())
                            });
                        }
                    }
                }
            }
        }
        
        Box::pin(async move {
            Err(actix_web::error::InternalError::from_response(
                "",
                HttpResponse::Unauthorized()
                    .json(serde_json::json!({
                        "message": "No valid authorization token found"
                    }))
            ).into())
        })
    }
}
