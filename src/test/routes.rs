use crate::api_error::ApiError;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use serde_json::json;

#[get("/test/get-param")]
async fn test_get_param(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let params = req.query_string();
    Ok(HttpResponse::Ok().json(params))
}

#[post("/test/post-param")]
async fn test_post_param(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    let params = req.query_string();
    Ok(HttpResponse::Ok().json(params))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(test_get_param);
    cfg.service(test_post_param);
}