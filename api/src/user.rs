use crate::db::Entity;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use macros::Entity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserLocation {
    pub suburb: String,
    pub postal_code: String,
    pub city: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "users"]
pub struct User {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    pub first_name: String,
    pub last_name: String,
    pub location: Option<UserLocation>,
    pub is_verified: bool,
    pub phone_number: Option<String>,
    pub email: String,

    #[serde(skip)]
    pub password_hash: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub location: Option<UserLocation>,
    pub phone_number: Option<String>,
    pub email: String,
    pub password: String,
}

impl From<NewUser> for User {
    fn from(value: NewUser) -> Self {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(value.password.as_bytes(), &salt)
            .expect("Couldn't hash password")
            .to_string();

        Self {
            id: None,
            first_name: value.first_name,
            last_name: value.last_name,
            location: value.location,
            is_verified: false,
            phone_number: value.phone_number,
            email: value.email,
            password_hash,
        }
    }
}
