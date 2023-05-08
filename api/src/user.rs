use crate::db::Entity;
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
