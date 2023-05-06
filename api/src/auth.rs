use std::{
    io::Read,
    time::{SystemTime, UNIX_EPOCH},
};

use jsonwebtoken::{Algorithm, EncodingKey, Header};
use rocket::serde::json::Json;
use rocket::Responder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthClaims {
    pub auth_type: AuthType,
    pub exp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum AuthType {
    User,
    Anonymous,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthRequest {
    pub auth_type: AuthType,
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
