use crate::api_error::ApiError;
use crate::permission::model::{Permission, PermissionMessage};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[get("/permissions")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let permissions = Permission::find_all()?;
    Ok(HttpResponse::Ok().json(permissions))
}

#[get("/permissions/{id}")]
async fn find(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let permission = Permission::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(permission))
}

#[post("/permissions")]
async fn create(permission: web::Json<PermissionMessage>) -> Result<HttpResponse, ApiError> {
    let permission = Permission::create(permission.into_inner())?;
    Ok(HttpResponse::Ok().json(permission))
}

#[put("/permissions/{id}")]
async fn update(id: web::Path<Uuid>, permission: web::Json<PermissionMessage>) -> Result<HttpResponse, ApiError> {
    let permission = Permission::update(id.into_inner(), permission.into_inner())?;
    Ok(HttpResponse::Ok().json(permission))
}

#[delete("/permissions/{id}")]
async fn delete(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let num_deleted = Permission::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}