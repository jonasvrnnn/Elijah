pub struct UserBase {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String
}

pub struct User {
    pub id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub totp_secret: Option<String>,
    pub role: String
}

pub struct Permission {
    pub company: Option<String>,
    pub create: Option<bool>,
    pub edit: Option<bool>
}