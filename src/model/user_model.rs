use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct SelectUserRequest {
    pub account: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub account: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub description: Option<String>,
    pub account: String,
    // TODO...
    pub old_password: String,
    pub new_password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteUserRequest {
    pub account: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub description: String,
    pub account: String,
    pub role_id: i32,
    pub ship_id: i32,
    pub created_at: String,
    pub updated_at: String,
}