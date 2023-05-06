use super::build_rocket;
use crate::auth::{AuthClaims, AuthRequest, AuthType, JWTAuthToken};
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;
use rocket::serde::json;
use rocket::uri;
use std::io::Read;

#[test]
fn hello_world() {
    let client = Client::tracked(build_rocket()).expect("valid rocket instance");
    let response = client.get(format!("/hello{}", uri!(super::hi))).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Hello World!");
}

#[test]
fn test_anonymous_auth() {
    let client = Client::tracked(build_rocket()).expect("valid rocket instance");
    let body = json::to_string(&AuthRequest {
        auth_type: crate::auth::AuthType::Anonymous,
    })
    .unwrap();

    let mut response = client
        .post(format!("/api{}", uri!(super::authenticate)))
        .header(ContentType::JSON)
        .body(body)
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
    let mut response_body = String::new();
    response.read_to_string(&mut response_body).unwrap();
    let response_body = json::from_str::<JWTAuthToken>(&response_body).unwrap();
    dbg!(&response_body);

    let mut public_key = String::new();
    std::fs::File::open("publicKey.pem")
        .unwrap()
        .read_to_string(&mut public_key)
        .unwrap();

    let claims: AuthClaims = jsonwebtoken::decode::<AuthClaims>(
        &response_body.token,
        &DecodingKey::from_rsa_pem(public_key.as_bytes()).expect("Expected valid decoding key"),
        &Validation::new(Algorithm::RS256),
    )
    .unwrap()
    .claims;

    assert_eq!(claims.auth_type, AuthType::Anonymous);
}
