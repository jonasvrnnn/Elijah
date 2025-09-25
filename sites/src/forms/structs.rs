use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormBody {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub message: String,
}