use crate::api::ApiError;
use crate::db::Entity;
use crate::user::User;
use crate::DB_NAME;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use bson::oid::ObjectId;
use bson::Document;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use log::warn;
use log::{error, info};
use macros::Entity;
use mongodb::Client;
use rand::{RngCore, SeedableRng};
use rocket::futures::TryStreamExt;
use rocket::http::{Cookie, CookieJar};
use rocket::request::{FromRequest, Outcome};
use rocket::serde::json::Json;
use rocket::Responder;
use rocket::{post, State};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::io::AsyncReadExt;
use utoipa::ToSchema;

#[utoipa::path(post, tag = "Authenticate", path = "/api/auth", request_body = AuthRequest)]
#[post("/auth", format = "application/json", data = "<auth_request>")]
pub async fn authenticate(
    auth_request: Json<AuthRequest>,
    state: &State<Option<Client>>,
    cookies: &CookieJar<'_>,
) -> Result<AuthResponder, Json<ApiError<'static>>> {
    match auth_request.auth_type {
        AuthType::Anonymous => Ok(AuthResponder {
            inner: Json(
                JWTAuthToken::new(auth_request.auth_type, None)
                    .await
                    .unwrap(),
            ),
            header: rocket::http::Header::new("X-Anon-Auth", "yes"),
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

            #[allow(deprecated)]
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
                            Ok(_) => {
                                let mut rng = rand::rngs::StdRng::from_entropy();
                                let mut cookie = vec![0u8; 32];
                                rng.fill_bytes(cookie.as_mut());
                                let cookie = AuthCookie::new(
                                    &cookie,
                                    user.id.expect("Couldn't fetch user id from database"),
                                );

                                if let Err(err) = cookie.insert(&db.database(DB_NAME)).await {
                                    log::error!("Couldn't write cookie to database: {err:?}");
                                    return Err(Json(ApiError::ServerError(
                                        "Couldn't communicate with the database",
                                    )));
                                }

                                Ok(AuthResponder {
                                    inner: Json(
                                        JWTAuthToken::new(auth_request.auth_type, Some(&user))
                                            .await
                                            .unwrap(),
                                    ),
                                    header: rocket::http::Header::new(
                                        "Set-Cookie",
                                        format!("cookie={cookie};expires=0;path=/;SameSite=Strict"),
                                    ),
                                })
                            }
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
        AuthType::Cookie => {
            let db = state.inner().as_ref().unwrap();
            if let Some(cookie) = cookies.get("cookie").map(Cookie::value) {
                log::info!("User trying to authenticate with cookie ({cookie})");
                #[allow(deprecated)]
                match AuthCookie::query(
                    bson::doc! {
                        "cookie": cookie
                    },
                    &db.database(DB_NAME),
                )
                .await
                {
                    Ok(mut cursor) => {
                        if let Ok(true) = cursor.advance().await {
                            let db_cookie = cursor.deserialize_current().unwrap();
                            if SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_secs()
                                > db_cookie.exp
                            {
                                return Err(Json(ApiError::AuthError("Expired cookie")));
                            }

                            let mut cursor = User::query(
                                bson::doc! {
                                    "_id": db_cookie.user
                                },
                                &db.database(DB_NAME),
                            )
                            .await
                            .expect("Couldn't query db to find user associated with cookie");

                            let user = if let Ok(true) = cursor.advance().await {
                                cursor.deserialize_current().unwrap()
                            } else {
                                return Err(Json(ApiError::AuthError(
                                    "Couldn't find user associated with cookie",
                                )));
                            };

                            return Ok(AuthResponder {
                                inner: Json(
                                    JWTAuthToken::new(AuthType::User, Some(&user))
                                        .await
                                        .expect("Couldn't generate JWT"),
                                ),
                                header: rocket::http::Header::new("X-User-Auth", "yes"),
                            });
                        } else {
                            return Err(Json(ApiError::AuthError(
                                "Couldn't find the cookie in the database",
                            )));
                        }
                    }
                    Err(err) => {
                        log::error!("Couldn't fetch cookie from database: {err:?}");
                        return Err(Json(ApiError::AuthError(
                            "Couldn't communicate with database",
                        )));
                    }
                }
            } else {
                Err(Json(ApiError::AuthError(
                    "Cookie auth type selected, but not cookie is present",
                )))
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthClaims {
    pub auth_type: AuthType,
    pub email: Option<String>,
    pub exp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, ToSchema)]
pub enum AuthType {
    User,
    Anonymous,
    Cookie,
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
#[serde(rename_all = "camelCase")]
pub struct JWTAuthToken {
    pub token: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for JWTAuthToken {
    type Error = ApiError<'static>;

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        use rocket::http::Status;
        let auth_header = request.headers().iter().find(|h| h.name == "Authorization");

        if auth_header.is_none() {
            return Outcome::Failure((
                Status::Unauthorized,
                ApiError::AuthError("No authorization header"),
            ));
        }

        let auth_header = auth_header.unwrap();
        let auth_header = auth_header.value.trim().split(" ").collect::<Vec<_>>();

        if auth_header.len() != 2 {
            Outcome::Failure((
                Status::Unauthorized,
                ApiError::AuthError("Unable to recover JWT from headers"),
            ))
        } else {
            let public_key = match tokio::fs::read_to_string("publicKey.pem").await {
                Ok(pk) => match DecodingKey::from_rsa_pem(pk.as_bytes()) {
                    Ok(pk) => pk,
                    Err(err) => {
                        log::error!("Couldn't decode public key: {err:?}");
                        return Outcome::Failure((
                            Status::InternalServerError,
                            ApiError::ServerError("We couldn't decode your auth token"),
                        ));
                    }
                },
                Err(err) => {
                    log::error!("Couldn't read public key for JWT decoding: {err:?}");
                    return Outcome::Failure((
                        Status::InternalServerError,
                        ApiError::ServerError("We couldn't decode your auth token"),
                    ));
                }
            };

            let mut validation = Validation::new(Algorithm::RS256);
            validation.leeway = 10;
            match jsonwebtoken::decode::<AuthClaims>(auth_header[1], &public_key, &validation) {
                Ok(claims) => match claims.claims {
                    AuthClaims {
                        auth_type: AuthType::User,
                        email,
                        ..
                    } => {
                        if email.is_none() {
                            error!("Received a user auth claim with no attached email. Something is wrong!");
                            return Outcome::Failure((
                                Status::Unauthorized,
                                ApiError::AuthError("Invalid token").into(),
                            ));
                        }

                        if let Some(client) = request.rocket().state::<Option<mongodb::Client>>() {
                            let mut doc = Document::new();
                            doc.insert("email", email.unwrap());
                            let user = if let Some(user) = User::find_one(
                                doc,
                                &client
                                    .as_ref()
                                    .expect("Couldn't connect to database")
                                    .database(DB_NAME),
                                None,
                            )
                            .await
                            {
                                user
                            } else {
                                return Outcome::Failure((
                                    Status::Unauthorized,
                                    ApiError::AuthError("Couldn't find the requested user"),
                                ));
                            };

                            Outcome::Success(JWTAuthToken {
                                token: auth_header[1].to_string(),
                                email: Some(user.email),
                                first_name: Some(user.first_name),
                                last_name: Some(user.last_name),
                            })
                        } else {
                            Outcome::Failure((
                                Status::InternalServerError,
                                ApiError::ServerError("Unable to communicate with database"),
                            ))
                        }
                    }
                    AuthClaims {
                        auth_type: AuthType::Anonymous,
                        ..
                    } => Outcome::Success(JWTAuthToken {
                        token: auth_header[1].to_string(),
                        email: None,
                        first_name: None,
                        last_name: None,
                    }),
                    _ => Outcome::Failure((
                        Status::Unauthorized,
                        ApiError::AuthError("Invalid token"),
                    )),
                },
                Err(_) => {
                    Outcome::Failure((Status::Unauthorized, ApiError::AuthError("Invalid token")))
                }
            }
        }
    }
}

#[derive(Responder)]
pub struct AuthResponder {
    pub inner: Json<JWTAuthToken>,
    pub header: rocket::http::Header<'static>,
}

async fn read_private_key(path: &str) -> Result<String, tokio::io::Error> {
    let mut file = tokio::fs::File::open(path).await?;
    let mut buff = String::new();
    file.read_to_string(&mut buff).await?;
    Ok(buff)
}

impl JWTAuthToken {
    pub async fn new(
        auth_type: AuthType,
        user: Option<&User>,
    ) -> Result<Self, jsonwebtoken::errors::Error> {
        let header = Header::new(Algorithm::RS256);

        let private_key = match read_private_key("privateKey.pem").await {
            Ok(key) => key,
            Err(err) => {
                warn!("Couldn't read private key file: {err:?}");
                std::env::var("JWT_PRIVATE_KEY").expect("Couldn't find a private key anywhere")
            }
        };

        let claims = AuthClaims {
            auth_type,
            email: user.map(|user| user.email.clone()),
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Couldn't get system time")
                .as_secs()
                + 3600 * 6,
        };

        log::info!(
            "Creating JWT for {}. Valid until {}",
            if let AuthType::User = claims.auth_type {
                format!("User({})", claims.email.clone().unwrap())
            } else {
                "Anonymous".to_string()
            },
            claims.exp.clone()
        );

        let token = jsonwebtoken::encode(
            &header,
            &claims,
            &EncodingKey::from_rsa_pem(private_key.as_bytes())
                .expect("Expected valid encoding key"),
        )?;

        Ok(Self {
            token,
            email: user.map(|x| x.email.clone()),
            first_name: user.map(|x| x.first_name.clone()),
            last_name: user.map(|x| x.last_name.clone()),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Entity)]
#[collection_name = "cookies"]
pub struct AuthCookie {
    #[serde(rename = "_id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    cookie: String,
    pub user: ObjectId,
    pub exp: u64,
}

impl AuthCookie {
    pub fn new(cookie: &impl AsRef<[u8]>, user: ObjectId) -> Self {
        Self {
            id: None,
            cookie: hex::encode(cookie.as_ref()),
            user,
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + 3600 * 24 * 7,
        }
    }
}

impl std::fmt::Display for AuthCookie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.cookie.as_ref())
    }
}
