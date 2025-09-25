use serde::{Deserialize, Serialize};

use crate::auth::models::User;

#[derive(Deserialize)]
pub struct LoginBody {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserDTO {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

impl UserDTO {
    fn from_user(user: User) -> Self {
        UserDTO {
            email: user.email,
            first_name: user.first_name,
            last_name: user.last_name,
        }
    }
}

#[derive(Deserialize)]
pub struct TotpVerifyBody {
    pub id: String,
    pub code: u32,
}

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub role: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct UpdateUserDataBody {
    pub email: Option<String>,
    pub role: Option<String>
}

#[derive(Deserialize)]
pub struct SetPermissionQuery {
    pub company: Option<String>
}

#[derive(Deserialize)]
pub struct SetPermissionBody {
    #[serde(default)]
    pub create: bool,
    #[serde(default)]
    pub edit: bool
}