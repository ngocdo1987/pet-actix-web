use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    // pub name: String,
    pub email: String,
    // pub access_token: String,
    // pub refresh_token: String,
}