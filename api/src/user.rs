use crate::{api::ApiError, db::Entity, DB_NAME};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use bson::Document;
use macros::Entity;
use mongodb::Client;
use rocket::{post, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[utoipa::path(post, path = "/api/user", request_body = NewUser, responses(
    (status = 200, description = "User creation result")
))]
#[post("/user", format = "application/json", data = "<new_user>")]
pub async fn create_user(
    state: &State<Option<Client>>,
    new_user: Json<NewUser>,
) -> Result<&'static str, Json<ApiError<'static>>> {
    if state.is_none() {
        return Err(Json(ApiError::ServerError(
            "Database is unavailable. Please try again later!",
        )));
    }

    let state = state.inner().as_ref().unwrap();

    let mut query = Document::new();
    query.insert("email", new_user.email.clone());
    let mut result = User::query(query, &state.database(DB_NAME))
        .await
        .expect("Couldn't query users");

    while result.advance().await.expect("Couldn't advance cursor") {
        let user = result
            .deserialize_current()
            .expect("Couldn't deserialize database user");
        if user.email == new_user.email {
            return Err(Json(ApiError::UserCreationError(
                "A user with that email already exists",
            )));
        }
    }

    User::from(new_user.into_inner())
        .insert(&state.database(DB_NAME))
        .await
        .expect("Couldn't insert new user!");

    Ok("User created")
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
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

    #[serde(skip_serializing_if = "String::is_empty")]
    pub password_hash: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
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
