use super::build_rocket;
use crate::auth::{AuthClaims, AuthRequest, AuthType, JWTAuthToken};
use crate::db::Entity;
use crate::user::User;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use rocket::serde::json;
use rocket::uri;
use tokio::io::AsyncReadExt;
use tokio::task::spawn_blocking;

#[rocket::async_test]
async fn hello_world() {
    let client = Client::tracked(build_rocket().await)
        .await
        .expect("valid rocket instance");
    let response = client
        .get(format!("/hello{}", uri!(super::hi)))
        .dispatch()
        .await;
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap(), "Hello World!");
}

#[rocket::async_test]
async fn test_anonymous_auth() {
    let client = Client::tracked(build_rocket().await)
        .await
        .expect("valid rocket instance");
    let body = json::to_string(&AuthRequest {
        auth_type: crate::auth::AuthType::Anonymous,
        email: None,
        password: None,
    })
    .unwrap();

    let mut response = client
        .post(format!("/api{}", uri!(super::authenticate)))
        .header(ContentType::JSON)
        .body(body)
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    let mut response_body = String::new();
    response.read_to_string(&mut response_body).await.unwrap();
    let response_body = json::from_str::<JWTAuthToken>(&response_body).unwrap();
    dbg!(&response_body);

    let public_key = tokio::fs::read_to_string("publicKey.pem").await.unwrap();

    let claims: AuthClaims = spawn_blocking(move || {
        jsonwebtoken::decode::<AuthClaims>(
            &response_body.token,
            &DecodingKey::from_rsa_pem(public_key.as_bytes()).expect("Expected valid decoding key"),
            &Validation::new(Algorithm::RS256),
        )
    })
    .await
    .unwrap()
    .unwrap()
    .claims;

    assert_eq!(claims.auth_type, AuthType::Anonymous);
}

#[rocket::async_test]
async fn test_create_user() {
    let rocket = build_rocket().await;
    let db = rocket.state::<mongodb::Client>().unwrap().database("wip");

    let client = Client::tracked(rocket)
        .await
        .expect("valid rocket instance");

    let new_user = User {
        first_name: String::from("Joe"),
        last_name: String::from("Rogan"),
        email: String::from("joe@theroganshow.com"),
        id: None,
        is_verified: false,
        phone_number: None,
        location: None,
        password_hash: String::new(),
    };

    let response = client
        .post(format!("/api{}", uri!(super::create_user)))
        .header(ContentType::JSON)
        .body(json::to_string(&new_user).unwrap())
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    // Revert any database actions that may have occurred
    new_user.delete(&db).await.unwrap();
}