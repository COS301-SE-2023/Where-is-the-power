use crate::api::ApiError;
use crate::db::Entity;
use crate::user::User;
use crate::DB_NAME;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use bson::Document;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use log::{error, info};
use mongodb::Client;
use rocket::futures::TryStreamExt;
use rocket::Responder;
use rocket::{post, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use std::{
    io::Read,
    time::{SystemTime, UNIX_EPOCH},
};
use utoipa::ToSchema;

#[utoipa::path(post, tag = "Authenticate", path = "/api/auth", request_body = AuthRequest)]
#[post("/auth", format = "application/json", data = "<auth_request>")]
pub async fn authenticate(
    auth_request: Json<AuthRequest>,
    state: &State<Option<Client>>,
) -> Result<AuthResponder, Json<ApiError<'static>>> {
    match auth_request.auth_type {
        AuthType::Anonymous => Ok(AuthResponder {
            inner: Json(JWTAuthToken::new(auth_request.auth_type).unwrap()),
            header: rocket::http::Header::new(
                "Set-Cookie",
                "cookie=some_cookie;expires=0;path=/;SameSite=Strict".to_string(),
            ),
        }),
        AuthType::User => {
            let db = state.inner().as_ref().unwrap();

            let email = auth_request.email.clone();

            if auth_request.password.is_none() {
                return Err(Json(ApiError::AuthError("Missing password")));
            }

            let password = auth_request.password.clone().unwrap();
            let mut doc = Document::new();
            doc.insert("email", email);

            match User::query(doc, &db.database(DB_NAME)).await {
                Ok(mut result) => match result.try_next().await {
                    Ok(user) => {
                        if user.is_none() {
                            return Err(Json(ApiError::AuthError("No such user")));
                        }
                        let user = user.unwrap();

                        let argon = Argon2::default();
                        let hash = PasswordHash::new(&user.password_hash).unwrap();
                        match argon.verify_password(password.as_bytes(), &hash) {
                            Ok(_) => Ok(AuthResponder {
                                inner: Json(JWTAuthToken::new(auth_request.auth_type).unwrap()),
                                header: rocket::http::Header::new(
                                    "Set-Cookie",
                                    "cookie=some_cookie;expires=0;path=/;SameSite=Strict"
                                        .to_string(),
                                ),
                            }),
                            Err(err) => {
                                info!("Password hash incorrect, rejecting user login: {err:?}");
                                Err(Json(ApiError::AuthError("Incorrect password")))
                            }
                        }
                    }
                    Err(err) => {
                        error!("Couldn't fetch user from result: {err:?}");
                        Err(Json(ApiError::ServerError("Couldn't resolve user")))
                    }
                },
                Err(err) => {
                    error!("Couldn't query database: {err:?}");
                    Err(Json(ApiError::ServerError("Couldn't query database")))
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthClaims {
    pub auth_type: AuthType,
    pub exp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, ToSchema)]
pub enum AuthType {
    User,
    Anonymous,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example = json! {
    AuthRequest {
        auth_type: AuthType::Anonymous,
        email: None,
        password: None
    }
})]
pub struct AuthRequest {
    pub auth_type: AuthType,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JWTAuthToken {
    pub token: String,
}

#[derive(Responder)]
pub struct AuthResponder {
    pub inner: Json<JWTAuthToken>,
    pub header: rocket::http::Header<'static>,
}

impl JWTAuthToken {
    pub fn new(auth_type: AuthType) -> Result<Self, jsonwebtoken::errors::Error> {
        let header = Header::new(Algorithm::RS256);

        let mut private_key_file =
            std::fs::File::open("privateKey.pem").expect("Expected private key file to exist");
        let mut private_key = String::new();
        private_key_file
            .read_to_string(&mut private_key)
            .expect("Expected to be able to read private key file");

        let claims = AuthClaims {
            auth_type,
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Couldn't get system time")
                .as_secs(),
        };

        let token = jsonwebtoken::encode(
            &header,
            &claims,
            &EncodingKey::from_rsa_pem(private_key.as_bytes())
                .expect("Expected valid encoding key"),
        )?;

        Ok(Self { token })
    }
}
