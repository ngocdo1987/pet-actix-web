use crate::api_error::ApiError;
use crate::db;
use crate::schema::permission;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "permission"]
pub struct PermissionMessage {
    pub name: String,
    pub guard_name: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "permission"]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub guard_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Permission {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let permissions = permission::table
            .load::<Permission>(&conn)?;

        Ok(permissions)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let permission = permission::table
            .filter(permission::id.eq(id))
            .first(&conn)?;

        Ok(permission)
    }

    pub fn create(permission: PermissionMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let permission = Permission::from(permission);
        let permission = diesel::insert_into(permission::table)
            .values(permission)
            .get_result(&conn)?;

        Ok(permission)
    }

    pub fn update(id: Uuid, permission: PermissionMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let permission = diesel::update(permission::table)
            .filter(permission::id.eq(id))
            .set(permission)
            .get_result(&conn)?;

        Ok(permission)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
                permission::table
                    .filter(permission::id.eq(id))
            )
            .execute(&conn)?;

        Ok(res)
    }
}

impl From<PermissionMessage> for Permission {
    fn from(permission: PermissionMessage) -> Self {
        Permission {
            id: Uuid::new_v4(),
            name: permission.name,
            guard_name: permission.guard_name,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}