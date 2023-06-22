use crate::api_error::ApiError;
use crate::role::model::{Role, RoleMessage};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;
use uuid::Uuid;

#[get("/roles")]
async fn find_all() -> Result<HttpResponse, ApiError> {
    let roles = Role::find_all()?;
    Ok(HttpResponse::Ok().json(roles))
}

#[get("/roles/{id}")]
async fn find(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let role = Role::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(role))
}

#[post("/roles")]
async fn create(role: web::Json<RoleMessage>) -> Result<HttpResponse, ApiError> {
    let role = Role::create(role.into_inner())?;
    Ok(HttpResponse::Ok().json(role))
}

#[put("/roles/{id}")]
async fn update(id: web::Path<Uuid>, role: web::Json<RoleMessage>) -> Result<HttpResponse, ApiError> {
    let role = Role::update(id.into_inner(), role.into_inner())?;
    Ok(HttpResponse::Ok().json(role))
}

#[delete("/roles/{id}")]
async fn delete(id: web::Path<Uuid>) -> Result<HttpResponse, ApiError> {
    let num_deleted = Role::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": num_deleted })))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(find);
    cfg.service(create);
    cfg.service(update);
    cfg.service(delete);
}