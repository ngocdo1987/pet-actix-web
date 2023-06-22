use crate::api_error::ApiError;
use crate::db;
use crate::schema::role;
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, AsChangeset)]
#[table_name = "role"]
pub struct RoleMessage {
    pub name: String,
    pub guard_name: String,
}

#[derive(Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "role"]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub guard_name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
}

impl Role {
    pub fn find_all() -> Result<Vec<Self>, ApiError> {
        let conn = db::connection()?;

        let roles = role::table
            .load::<Role>(&conn)?;

        Ok(roles)
    }

    pub fn find(id: Uuid) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let role = role::table
            .filter(role::id.eq(id))
            .first(&conn)?;

        Ok(role)
    }

    pub fn create(role: RoleMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let role = Role::from(role);
        let role = diesel::insert_into(role::table)
            .values(role)
            .get_result(&conn)?;

        Ok(role)
    }

    pub fn update(id: Uuid, role: RoleMessage) -> Result<Self, ApiError> {
        let conn = db::connection()?;

        let role = diesel::update(role::table)
            .filter(role::id.eq(id))
            .set(role)
            .get_result(&conn)?;

        Ok(role)
    }

    pub fn delete(id: Uuid) -> Result<usize, ApiError> {
        let conn = db::connection()?;

        let res = diesel::delete(
                role::table
                    .filter(role::id.eq(id))
            )
            .execute(&conn)?;

        Ok(res)
    }
}

impl From<RoleMessage> for Role {
    fn from(role: RoleMessage) -> Self {
        Role {
            id: Uuid::new_v4(),
            name: role.name,
            guard_name: role.guard_name,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
        }
    }
}