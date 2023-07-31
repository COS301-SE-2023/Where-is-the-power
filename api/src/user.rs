use std::collections::HashMap;

use crate::{
    api::{ApiError, ApiResponse},
    auth::JWTAuthToken,
    db::Entity,
    DB_NAME,
};
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use bson::{oid::ObjectId, Document};
use macros::Entity;
use mongodb::Client;
use rocket::{delete, get, post, put, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[utoipa::path(post, tag = "Create User", path = "/api/user", request_body = NewUser, responses(
    (status = 200, description = "User creation result", body = ResponseString)
))]
#[post("/user", format = "application/json", data = "<new_user>")]
pub async fn create_user(
    state: &State<Option<Client>>,
    new_user: Json<NewUser>,
) -> ApiResponse<&'static str> {
    if state.is_none() {
        return ApiError::ServerError("Database is unavailable. Please try again later!").into();
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
            return ApiError::UserCreationError("A user with that email already exists").into();
        }
    }

    User::from(new_user.into_inner())
        .insert(&state.database(DB_NAME))
        .await
        .expect("Couldn't insert new user!");

    ApiResponse::Ok("User created")
}

#[utoipa::path(put, path = "/api/user/savedPlaces", request_body = SavedPlace, security(("jwt" = [])))]
#[put(
    "/user/savedPlaces",
    format = "application/json",
    data = "<saved_place>"
)]
pub async fn add_saved_place(
    token: JWTAuthToken,
    saved_place: Json<SavedPlace>,
    state: &State<Option<mongodb::Client>>,
) -> ApiResponse<&'static str> {
    if token.email.is_none() {
        return ApiError::AuthError("Authenticated user required").into();
    }

    let email = token.email.unwrap();
    let db = state.as_ref().unwrap().database(DB_NAME);
    let mut doc = Document::new();
    doc.insert("email", email);

    let mut user = dbg!(if let Ok(mut user) = User::query(doc, &db).await {
        user.advance().await.expect("Couldn't fetch user!");
        user.deserialize_current().unwrap()
    } else {
        return ApiError::ServerError("The requested user could not be found").into();
    });

    if user.saved_places.contains_key(&saved_place.mapbox_id) {
        return ApiError::SavedPlacesError("Duplicate saved place").into();
    }

    user.saved_places
        .insert(saved_place.mapbox_id.clone(), saved_place.into_inner());

    let places = dbg!(mongodb::bson::to_bson(&user.saved_places).unwrap());
    let doc = mongodb::bson::doc! {
        "$set": {
            "savedPlaces": places
        }
    };

    match user.update(doc.into(), &db).await {
        Ok(_) => ApiResponse::Ok("New Saved Place recorded"),
        Err(err) => {
            log::error!("Couldn't update user: {err:?}");
            ApiError::ServerError("Unable to record new saved place").into()
        }
    }
}

#[utoipa::path(get, path = "/api/user/savedPlaces", security(("jwt" = [])))]
#[get("/user/savedPlaces")]
pub async fn get_saved_places(
    token: JWTAuthToken,
    state: &State<Option<mongodb::Client>>,
) -> ApiResponse<Vec<SavedPlace>> {
    if token.email.is_none() {
        return ApiError::AuthError("Only authenticated users can use this endpoint").into();
    }

    let email = token.email.unwrap();
    let doc = bson::doc! {
        "email": email
    };

    let db = state.as_ref().unwrap().database(DB_NAME);
    if let Ok(mut cursor) = User::query(doc, &db).await {
        if let Ok(true) = cursor.advance().await {
            let user = cursor.deserialize_current().unwrap();
            ApiResponse::Ok(
                user.saved_places
                    .into_iter()
                    .map(|(_, v)| v)
                    .collect::<Vec<_>>(),
            )
        } else {
            ApiError::ServerError("Couldn't find your user").into()
        }
    } else {
        ApiError::ServerError("Couldn't query the database for your user").into()
    }
}

#[utoipa::path(delete, path = "/api/user/savedPlaces/{id}", params(("id",)), security(("jwt" = [])))]
#[delete("/user/savedPlaces/<id>")]
pub async fn delete_saved_place(
    id: &str,
    token: JWTAuthToken,
    state: &State<Option<mongodb::Client>>,
) -> ApiResponse<'static, &'static str> {
    if token.email.is_none() {
        return ApiError::AuthError("This endpoint is only available to logged in users").into();
    }

    let email = token.email.unwrap();
    let doc = bson::doc! {
        "email":  email
    };

    let db = state.as_ref().unwrap().database(DB_NAME);
    let mut user = dbg!(match User::query(doc, &db).await {
        Ok(mut cursor) => {
            if let Ok(true) = cursor.advance().await {
                cursor.deserialize_current().unwrap()
            } else {
                return ApiError::AuthError("Couldn't find your user").into();
            }
        }
        Err(err) => {
            log::error!("Couldn't fetch user from database: {err:?}");
            return ApiError::ServerError("Couldn't query the database to find your user").into();
        }
    });

    if !user.saved_places.contains_key(id) {
        ApiError::SavedPlacesError("The provided id doesn't exist").into()
    } else {
        user.saved_places.remove(id);
        let places = bson::to_document(&user.saved_places).unwrap();
        match user
            .update(
                bson::doc! {
                     "$set": {
                        "savedPlaces": places
                    }
                }
                .into(),
                &db,
            )
            .await
        {
            Ok(_) => ApiResponse::Ok("Saved Place deleted"),
            Err(err) => {
                log::error!("Couldn't delete saved place: {err:?}");
                ApiError::ServerError("Couldn't delete saved place").into()
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserLocation {
    pub suburb: String,
    pub postal_code: String,
    pub city: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SavedPlace {
    pub mapbox_id: String,
    pub name: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Entity)]
#[serde(rename_all = "camelCase")]
#[collection_name = "users"]
pub struct User {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub first_name: String,
    pub last_name: String,
    pub location: Option<UserLocation>,
    pub is_verified: bool,
    pub phone_number: Option<String>,
    pub email: String,
    pub saved_places: HashMap<String, SavedPlace>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub password_hash: String,
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = r#"
    {
        "firstName": "Joe",
        "lastName": "Average",
        "email": "joe@average.net",
        "password": "super_secure_p@ssword"
    }
"#)]
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
            saved_places: HashMap::new(),
            password_hash,
        }
    }
}
