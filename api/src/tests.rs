use super::build_rocket;
use crate::auth::{AuthClaims, AuthRequest, AuthType, JWTAuthToken};
use crate::loadshedding::{MockDBFunctions, GroupEntity, TimeScheduleEntity};
use crate::scraper::convert_to_ints;
// use crate::db::Entity;
// use crate::user::User;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use rocket::serde::json::{self};
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
        .post(format!("/api{}", uri!(super::auth::authenticate)))
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
async fn test_find_user() {}

#[test]
fn time_range_validation_test_fails() {
    let cases = vec![
        "giberish",
        "giber-ish",
        "gi:ber-I:sh",
        "1:3b-e:r",
        "1::30-0:00",
        ":1:2-23",
        "1-2",
        "23:60-22:00",
    ];
    let _ = cases
        .iter()
        .map(|case| {
            let result = convert_to_ints(case);
            assert!(result.is_err(), "Failed to protect against poluted Data")
        })
        .collect::<Vec<_>>();
}

#[test]
fn time_range_validation_test_pass() {
    let cases = vec![
        "1:20 - 2:30",
        "  2:3 - 3: 0",
        "22:30-2:03",
        "1 : 3 - 2 : 3",
        "23:59-00:00",
    ];
    let _ = cases
        .iter()
        .map(|case| {
            let result = convert_to_ints(case);
            assert!(result.is_ok(), "False Negative")
        })
        .collect::<Vec<_>>();
}

#[test]
fn polygon_request_test() {
    let mut mock = MockDBFunctions::new();
    let data = r#"[{ "_id": { "$oid": "64b6b9b30d09aa7756061c0d" }, "startHour": 20, "startMinute": 0, "stopHour": 22, "stopMinute": 30, "stages": [ { "stage": 7, "groups": [ { "$oid": "64b6b9b30d09aa7756061bc9" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, ] }, { "stage": 3, "groups": [ { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b00" }, ] }, { "stage": 4, "groups": [ { "$oid": "64b6b9b30d09aa7756061aea" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, ] }, { "stage": 8, "groups": [ { "$oid": "64b6b9b30d09aa7756061b49" }, { "$oid": "64b6b9b30d09aa7756061bc9" }, ] }, { "stage": 2, "groups": [ { "$oid": "64b6b9b30d09aa7756061b00" }, { "$oid": "64b6b9b30d09aa7756061acd" }, ] }, { "stage": 6, "groups": [ { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061b7a" }, ] }, { "stage": 5, "groups": [ { "$oid": "64b6b9b30d09aa7756061b7a" }, { "$oid": "64b6b9b30d09aa7756061b49" }, ] }, { "stage": 1, "groups": [ { "$oid": "64b6b9b30d09aa7756061acd" }, { "$oid": "64b6b9b30d09aa7756061aea" }, ] } ], "municipality": { "$oid": "64b6b9b30d09aa7756061a47" } }] "#;
    let test_schedule: Vec<TimeScheduleEntity> = serde_json::from_str(data).unwrap();
    let data = r#"[{ "_id": { "$oid": "64b6b9b30d09aa7756061b9d" }, "number": 3, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061b7b" }, ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061a79" }, "number": 4, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061a48" }, ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061a94" }, "number": 16, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061a7a" }, ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061ab6" }, "number": 14, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061a95" }, ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061acd" }, "number": 11, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061ab7" }, ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061aea" }, "number": 7, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061ace" }, ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061b00" }, "number": 15, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061aeb" }, ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061b2e" }, "number": 13, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061b01" }, ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061b49" }, "number": 6, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061b2f" }, ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061b7a" }, "number": 10, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061b4a" }, ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061ba9" }, "number": 12, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061b9e" }, ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061bc9" }, "number": 2, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061baa" }, ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061bcb" }, "number": 1, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061bca" } ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061bcd" }, "number": 9, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061bcc" } ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061be2" }, "number": 5, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061bce" }, ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061c0c" }, "number": 8, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061be3" }, ] } ] "#;
    let test_groups: Vec<GroupEntity> = serde_json::from_str(data).unwrap();
    let data = r#""#;
    mock.expect_collect_schedule()
        .returning(move |_,_,_| {
            Ok(test_schedule.clone())
        });
    mock.expect_collect_groups()
        .returning(move|_query, _conn, _opts| {
           Ok(test_groups.clone())
        });
}

// #[rocket::async_test]
// async fn test_create_user() {
//     let rocket = build_rocket().await;
//     let db = rocket.state::<mongodb::Client>().unwrap().database("wip");
//
//     let client = Client::tracked(rocket)
//         .await
//         .expect("valid rocket instance");
//
//     let new_user = User {
//         first_name: String::from("Joe"),
//         last_name: String::from("Rogan"),
//         email: String::from("joe@theroganshow.com"),
//         id: None,
//         is_verified: false,
//         phone_number: None,
//         location: None,
//         password_hash: String::new(),
//     };
//
//     let response = client
//         .post(format!("/api{}", uri!(super::create_user)))
//         .header(ContentType::JSON)
//         .body(json::to_string(&new_user).unwrap())
//         .dispatch()
//         .await;
//
//     assert_eq!(response.status(), Status::Ok);
//
//     // Revert any database actions that may have occurred
//     new_user.delete(&db).await.unwrap();
// }
