use super::build_rocket;
use bson::doc;
use chrono::{Utc, NaiveDateTime, DateTime};
use crate::ai::{AiInfoRequest, AiInfoResponse};
use crate::api::UnifiedResponse;
use crate::auth::{AuthClaims, AuthRequest, AuthType, JWTAuthToken};
use crate::loadshedding::{
    GroupEntity, MockDBFunctionsTrait, MunicipalityEntity, SuburbEntity, TimeScheduleEntity, LoadSheddingStage, SuburbStatsResponse, PredictiveSuburbStatsResponse, LoadsheddingData, SASTDateTime, DBFunctionsTrait,
};
use crate::scraper::convert_to_ints;
use jsonwebtoken::{Algorithm, DecodingKey, Validation};
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use rocket::serde::json::{self};
use rocket::uri;
use tokio::io::AsyncReadExt;
use tokio::task::spawn_blocking;

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
        "23:59-24:00",
    ];
    let _ = cases
        .iter()
        .map(|case| {
            let result = convert_to_ints(case);
            assert!(result.is_ok(), "False Negative")
        })
        .collect::<Vec<_>>();
}

#[rocket::async_test]
async fn polygon_request_test() {
    let test_municipality: MunicipalityEntity = serde_json::from_str(POLYGON_DATA).unwrap();
    let mock = create_mock();
    // the time is the first day of a month matching the time_schedule
    let result = test_municipality
        .get_regions_at_time(2, Some(1688237449), None, &mock)
        .await
        .unwrap();
    assert_eq!(
        result
            .map_polygons
            .get(0)
            .unwrap()
            .features
            .get(0)
            .unwrap()
            .properties
            .power_status,
        Some("off".to_string())
    );
    assert_eq!(
        result
            .map_polygons
            .get(0)
            .unwrap()
            .features
            .get(1)
            .unwrap()
            .properties
            .power_status,
        Some("off".to_string())
    );
    assert_eq!(
        result
            .map_polygons
            .get(0)
            .unwrap()
            .features
            .get(2)
            .unwrap()
            .properties
            .power_status,
        Some("undefined".to_string())
    );
    assert_eq!(
        result
            .map_polygons
            .get(0)
            .unwrap()
            .features
            .get(3)
            .unwrap()
            .properties
            .power_status,
        Some("on".to_string())
    );
}

#[rocket::async_test]
async fn test_buildschedule() {
    let testing_time = 1694660400;
    let testing_suburb: SuburbEntity = serde_json::from_str(TEST_SUBURB_DATA).unwrap();
    let mock = create_mock();
    let result = testing_suburb.build_schedule(None, &mock, Some(testing_time)).await.unwrap();
    let expected_output:PredictiveSuburbStatsResponse = serde_json::from_str(TEST_GETSCHEDULE_EXPECTED_RESULT).unwrap();
    println!("{:?}" , serde_json::to_string(&result).unwrap());
    assert_eq!(result,expected_output);
}

#[rocket::async_test]
async fn test_getstats() {
    let testing_time = 1695265200;
    let testing_suburb: SuburbEntity = serde_json::from_str(TEST_SUBURB_DATA).unwrap();
    let mock = create_mock();
    let result = testing_suburb.get_total_time_down_stats(None, &mock, Some(testing_time)).await.unwrap();
    let expected_output:SuburbStatsResponse = serde_json::from_str(TEST_GETSTATS_EXPECTED_RESULT).unwrap();
    //println!("{:?}", serde_json::to_string(&result).unwrap());
    assert_eq!(result,expected_output);
}

#[rocket::async_test]
async fn test_ai_endpoint() {
    let client = Client::tracked(build_rocket().await)
        .await
        .expect("valid rocket instance");

    let response = client
        .post(format!("/api{}", uri!(super::ai::get_ai_info)))
        .header(ContentType::JSON)
        .json(&AiInfoRequest {
            origin: Box::new([28.3, -27.73]),
            destination: Box::new([28.2651, -25.7597]),
        })
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().await.unwrap();
    let body = serde_json::from_str::<UnifiedResponse<AiInfoResponse>>(&body).unwrap();

    assert!(body.success);
}

#[rocket::async_test]
async fn test_loadshedding_helpers() {
    let mock = create_mock();
    let db: &dyn DBFunctionsTrait = &mock;
    let start = SASTDateTime(DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(1694660400, 0).unwrap(), Utc).fixed_offset());
    let end = SASTDateTime(DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp_opt(1694746800, 0).unwrap(), Utc).fixed_offset());
    let test_schedule_conversion = LoadsheddingData {
        stage: 0,
        start: start,
        end: end
    };
    let conversion = test_schedule_conversion.convert_to_loadsheddingstage();
    let doc = doc! {};
    let compare = db.collect_one_stage_log(doc, None, None).await.unwrap();
    assert_eq!(conversion.start_time, compare.start_time);
    assert_eq!(conversion.end_time, compare.end_time);
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

// DO NOT CHANGE ONCE FINALIZED
fn create_mock() -> MockDBFunctionsTrait  {
    let mut mock = MockDBFunctionsTrait::new();
    // DO NOT CHANGE ONCE FINALIZED
    let data = r#"[
        {
            "_id": { "$oid": "64b6b9b30d09aa7756061c0d" },
            "startHour": 20,
            "startMinute": 0,
            "stopHour": 22,
            "stopMinute": 30,
            "stages": [
                { "stage": 1, "groups": [ { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" } ] },
                { "stage": 2, "groups": [ { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }] },
                { "stage": 3, "groups": [ { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }] },
                { "stage": 4, "groups": [ { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }] }
            ],
            "municipality": { "$oid": "64b6b9b30d09aa7756061a47" }
        },
        {
            "_id": { "$oid": "64b6b9b30d09bb4756061c0d" },
            "startHour": 22,
            "startMinute": 0,
            "stopHour": 0,
            "stopMinute": 30,
            "stages": [
                { "stage": 2, "groups": [ { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" } ] },
                { "stage": 3, "groups": [ { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }] },
                { "stage": 4, "groups": [ { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }] },
                { "stage": 1, "groups": [ { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }] }
            ],
            "municipality": { "$oid": "64b6b9b30d09aa7756061a47" }
        },
        {
            "_id": { "$oid": "64b6b9b30dabbb4756061c0d" },
            "startHour": 0,
            "startMinute": 0,
            "stopHour": 2,
            "stopMinute": 30,
            "stages": [
                { "stage": 3, "groups": [ { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" } ] },
                { "stage": 4, "groups": [ { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }] },
                { "stage": 1, "groups": [ { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }] },
                { "stage": 2, "groups": [ { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }] }
            ],
            "municipality": { "$oid": "64b6b9b30d09aa7756061a47" }
        },
        {
            "_id": { "$oid": "63b6b9b30d09bb4756061c0d" },
            "startHour": 2,
            "startMinute": 0,
            "stopHour": 4,
            "stopMinute": 30,
            "stages": [
                { "stage": 4, "groups": [ { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" } ] },
                { "stage": 1, "groups": [ { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }] },
                { "stage": 2, "groups": [ { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }] },
                { "stage": 3, "groups": [ { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }] }
            ],
            "municipality": { "$oid": "64b6b9b30d09aa7756061a47" }
        },
        {
            "_id": { "$oid": "63b6b9bdcd09bb4756061c0d" },
            "startHour": 4,
            "startMinute": 0,
            "stopHour": 6,
            "stopMinute": 30,
            "stages": [
                { "stage": 1, "groups": [ { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" } ] },
                { "stage": 2, "groups": [ { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }] },
                { "stage": 3, "groups": [ { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }] },
                { "stage": 4, "groups": [ { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }] }
            ],
            "municipality": { "$oid": "64b6b9b30d09aa7756061a47" }
        },
        {
            "_id": { "$oid": "63b6b9b30d09bb475606dafd" },
            "startHour": 6,
            "startMinute": 0,
            "stopHour": 8,
            "stopMinute": 30,
            "stages": [
                { "stage": 2, "groups": [ { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" } ] },
                { "stage": 3, "groups": [ { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }] },
                { "stage": 4, "groups": [ { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }] },
                { "stage": 1, "groups": [ { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }] }
            ],
            "municipality": { "$oid": "64b6b9b30d09aa7756061a47" }
        },
        {
            "_id": { "$oid": "63b6b9b30d09cc4756061c0d" },
            "startHour": 8,
            "startMinute": 0,
            "stopHour": 10,
            "stopMinute": 30,
            "stages": [
                { "stage": 3, "groups": [ { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" } ] },
                { "stage": 4, "groups": [ { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }] },
                { "stage": 1, "groups": [ { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }] },
                { "stage": 2, "groups": [ { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }] }
            ],
            "municipality": { "$oid": "64b6b9b30d09aa7756061a47" }
        },
        {
            "_id": { "$oid": "63b6b9b30d09bcf756061c0d" },
            "startHour": 10,
            "startMinute": 0,
            "stopHour": 12,
            "stopMinute": 30,
            "stages": [
                { "stage": 4, "groups": [ { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" } ] },
                { "stage": 1, "groups": [ { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }] },
                { "stage": 2, "groups": [ { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }] },
                { "stage": 3, "groups": [ { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }] }
            ],
            "municipality": { "$oid": "64b6b9b30d09aa7756061a47" }
        },
        {
            "_id": { "$oid": "63b7b9b30d09bcf756061c0d" },
            "startHour": 12,
            "startMinute": 0,
            "stopHour": 14,
            "stopMinute": 30,
            "stages": [
                { "stage": 1, "groups": [ { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" } ] },
                { "stage": 2, "groups": [ { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }] },
                { "stage": 3, "groups": [ { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }] },
                { "stage": 4, "groups": [ { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }] }
            ],
            "municipality": { "$oid": "64b6b9b30d09aa7756061a47" }
        },
        {
            "_id": { "$oid": "63b6b1b30d09bcf756061c0d" },
            "startHour": 14,
            "startMinute": 0,
            "stopHour": 16,
            "stopMinute": 30,
            "stages": [
                { "stage": 2, "groups": [ { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" } ] },
                { "stage": 3, "groups": [ { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }] },
                { "stage": 4, "groups": [ { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }] },
                { "stage": 1, "groups": [ { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }] }
            ],
            "municipality": { "$oid": "64b6b9b30d09aa7756061a47" }
        },
        {
            "_id": { "$oid": "63b6b9b3dd09bcf756061c0d" },
            "startHour": 16,
            "startMinute": 0,
            "stopHour": 18,
            "stopMinute": 30,
            "stages": [
                { "stage": 3, "groups": [ { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" } ] },
                { "stage": 4, "groups": [ { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }] },
                { "stage": 1, "groups": [ { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }] },
                { "stage": 2, "groups": [ { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }] }
            ],
            "municipality": { "$oid": "64b6b9b30d09aa7756061a47" }
        },
        {
            "_id": { "$oid": "63b6b9b30d09bcf756061c0d" },
            "startHour": 18,
            "startMinute": 0,
            "stopHour": 20,
            "stopMinute": 30,
            "stages": [
                { "stage": 4, "groups": [ { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" }, { "$oid": "64b6b9b30d09aa7756061b9d" } ] },
                { "stage": 1, "groups": [ { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }, { "$oid": "64b6b9b30d09aa7756061a79" }] },
                { "stage": 2, "groups": [ { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061a94" }] },
                { "stage": 3, "groups": [ { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }, { "$oid": "64b6b9b30d09aa7756061ab6" }] }
            ],
            "municipality": { "$oid": "64b6b9b30d09aa7756061a47" }
        }
    ] "#;
    let test_schedule: Vec<TimeScheduleEntity> = serde_json::from_str(data).unwrap();
    // DO NOT CHANGE ONCE FINALIZED
    // STATUS = FINALIZED
    let data = r#"[
        {
            "_id": { "$oid": "64b6b9b30d09aa7756061b9d" },
            "number": 1,
            "suburbs": [
                { "$oid": "64b6b9b30d09aa7756061b30" }
            ]
        },{
            "_id": { "$oid": "64b6b9b30d09aa7756061a79" },
            "number": 2,
            "suburbs": [
                { "$oid": "64b6b9b30d09aa7756061b7d" }
            ]
        }, {
            "_id": { "$oid": "64b6b9b30d09aa7756061a94" },
            "number": 3,
            "suburbs": [ { "$oid": "64b6b9b30d09aa7756061a63" }]
        }
    ]"#;
    let test_groups: Vec<GroupEntity> = serde_json::from_str(data).unwrap();
    // DO NOT CHANGE ONCE FINALIZED
    // STATUS = FINALIZED
    let data = r#"[
        {
            "_id": { "$oid": "64b6b9b30d09aa7756061b30" },
            "municipality": { "$oid": "64b6b9b30d09aa7756061a47" },
            "name": "MUCKLENEUK",
            "geometry": [ 1 ]
        }, {
            "_id": { "$oid": "64b6b9b30d09aa7756061b7d" },
            "municipality": { "$oid": "64b6b9b30d09aa7756061a47" },
            "name": "NEWLANDS",
            "geometry": [ 2 ]
        }, {
            "_id": { "$oid": "64b6b9b30d09aa7756061a63" },
            "municipality": { "$oid": "64b6b9b30d09aa7756061a47" },
            "name": "SOSHANGUVE EAST",
            "geometry": [   ]
        }, {
            "_id": { "$oid": "64b6b9b30d09aa7756061a7b" },
            "municipality": { "$oid": "64b6b9b30d09aa7756061a47" },
            "name": "MAGALIESKRUIN",
            "geometry": [ 4 ]
        }
    ]"#;
    let test_suburbs: Vec<SuburbEntity> = serde_json::from_str(data).unwrap();

    let data = r#"[
    {
        "_id": {
            "$oid": "650b1a741329313fc8b0c15b"
        },
        "startTime": 1694746800,
        "endTime": 1694779200,
        "stage": 5
    },
    {
        "_id": {
            "$oid": "650b1a741329313fc8b0c15a"
        },
        "startTime": 1694822400,
        "endTime": 1694833200,
        "stage": 0
    },
    {
        "_id": {
            "$oid": "650b1a741329313fc8b0c159"
        },
        "startTime": 1694833200,
        "endTime": 1694854800,
        "stage": 2
    },
    {
        "_id": {
            "$oid": "650b1a741329313fc8b0c158"
        },
        "startTime": 1694854800,
        "endTime": 1694872800,
        "stage": 0
    },
    {
        "_id": {
            "$oid": "650b1a741329313fc8b0c157"
        },
        "startTime": 1694872800,
        "endTime": 1694959200,
        "stage": 0
    },
    {
        "_id": {
            "$oid": "650b1a741329313fc8b0c156"
        },
        "startTime": 1694959200,
        "endTime": 1695031200,
        "stage": 2
    },
    {
        "_id": {
            "$oid": "650b1a741329313fc8b0c155"
        },
        "startTime": 1695031200,
        "endTime": 1695045600,
        "stage": 1
    },
    {
        "_id": {
            "$oid": "650b1a741329313fc8b0c154"
        },
        "startTime": 1695045600,
        "endTime": 1695092400,
        "stage": 3
    },
    {
        "_id": {
            "$oid": "650b1a741329313fc8b0c153"
        },
        "startTime": 1695092400,
        "endTime": 1695132000,
        "stage": 0
    },
    {
        "_id": {
            "$oid": "650b1a741329313fc8b0c152"
        },
        "startTime": 1695132000,
        "endTime": 1695150000,
        "stage": 3
    },
    {
        "_id": {
            "$oid": "650b1a741329313fc8b0c151"
        },
        "startTime": 1695153600,
        "endTime": 1695160800,
        "stage": 1
    },
    {
        "_id": {
            "$oid": "650b1a741329313fc8b0c150"
        },
        "startTime": 1695160800,
        "endTime": 1695178800,
        "stage": 0
    },
    {
        "_id": {
            "$oid": "650b1a741329313fc8b0c14f"
        },
        "startTime": 1695178800,
        "endTime": 1695218400,
        "stage": 0
    },
    {
        "_id": {
            "$oid": "650c24c257de8d37915d203c"
        },
        "startTime": 1695254400,
        "endTime": 1695265200,
        "stage": 1
    },
    {
        "_id": {
            "$oid": "650b1a741329313fc8b0c14d"
        },
        "startTime": 1695265200,
        "endTime": 1695304800,
        "stage": 0
    },
    {
        "_id": {
            "$oid": "650b1a741329313fc8b0c14c"
        },
        "startTime": 1695304800,
        "endTime": 1695351600,
        "stage": 3
    }]"#;
    let test_stage_logs: Vec<LoadSheddingStage> = serde_json::from_str(data).unwrap();

    let data = r#"{
        "_id": {
        "$oid": "650b1a741329313fc8b0c15c"
        },
        "startTime": 1694660400,
        "endTime": 1694746800,
        "stage": 6
    }"#;
    let test_stage_log: LoadSheddingStage = serde_json::from_str(data).unwrap();

    let data = r#"{
        "_id": { "$oid": "64b6b9b30d09aa7756061b9d" },
        "number": 1,
        "suburbs": [
            { "$oid": "64b6b9b30d09aa7756061b30" }
        ]
    }"#;
    let test_one_group: GroupEntity = serde_json::from_str(data).unwrap();

    mock.expect_collect_schedules()
        .returning(move |_, _, _| Ok(test_schedule.clone()));
    mock.expect_collect_groups()
        .returning(move |_query, _conn, _opts| Ok(test_groups.clone()));
    mock.expect_collect_one_group()
        .returning(move |_query, _conn, _opts| Ok(test_one_group.clone()));
    mock.expect_collect_suburbs()
        .returning(move |_query, _conn, _opts| Ok(test_suburbs.clone()));
    mock.expect_collect_stage_logs()
        .returning(move |_query, _conn, _opts| Ok(test_stage_logs.clone()));
    mock.expect_collect_one_stage_log()
        .returning(move |_query, _conn, _opts| Ok(test_stage_log.clone()));
    mock
}
const TEST_SUBURB_DATA: &'static str = r#"{
    "_id": { "$oid": "64b6b9b30d09aa7756061b30" },
    "municipality": { "$oid": "64b6b9b30d09aa7756061a47" },
    "name": "MUCKLENEUK",
    "geometry": [ 1 ]
}"#;

const TEST_GETSTATS_EXPECTED_RESULT: &'static str = "{\"totalTime\":{\"on\":2520,\"off\":7560},\"perDayTimes\":{\"Sun\":{\"on\":360,\"off\":1080},\"Mon\":{\"on\":360,\"off\":1080},\"Sat\":{\"on\":360,\"off\":1080},\"Tue\":{\"on\":360,\"off\":1080},\"Thu\":{\"on\":360,\"off\":1080},\"Fri\":{\"on\":360,\"off\":1080},\"Wed\":{\"on\":360,\"off\":1080}},\"suburb\":{\"_id\":{\"$oid\":\"64b6b9b30d09aa7756061b30\"},\"municipality\":{\"$oid\":\"64b6b9b30d09aa7756061a47\"},\"name\":\"MUCKLENEUK\",\"geometry\":[1]}}";
const TEST_GETSCHEDULE_EXPECTED_RESULT: &'static str = "{\"timesOff\":[{\"start\":1694656800,\"end\":1694723400},{\"start\":1694728800,\"end\":1694752200}]}";
const POLYGON_DATA: &'static str = r#"{
    "_id": { "$oid": "64b6b9b30d09aa7756061a47" },
    "name": "tshwane",
    "geometry": {
        "name": "SP_SA_2011|Selection",
        "map_layer_type": "Area",
        "bounds": [
            [ 27.890227, -26.077549 ],
            [ 29.098541, -25.110155 ]
        ],
        "center": [ 28.494384, -25.593852 ],
        "zoom": 6,
        "median_zoom": 13,
        "count": 594,
        "property_names": [
            "SP_CODE",
            "SP_CODE_st",
            "SP_NAME",
            "MP_CODE",
            "MP_CODE_st",
            "MP_NAME",
            "MN_MDB_C",
            "MN_CODE",
            "MN_CODE_st",
            "MN_NAME",
            "DC_MDB_C",
            "DC_MN_C",
            "DC_MN_C_st",
            "DC_NAME",
            "PR_MDB_C",
            "PR_CODE",
            "PR_CODE_st",
            "PR_NAME",
            "ALBERS_ARE",
            "Shape_Leng",
            "Shape_Area"
        ],
        "type": "FeatureCollection",
        "features": [
            {
                "type": "Feature",
                "id": 1,
                "properties": {
                    "SP_CODE": 799078003,
                    "SP_CODE_st": "799078003",
                    "SP_NAME": "MUCKLNEUK",
                    "MP_CODE": 799078,
                    "MP_CODE_st": "799078",
                    "MP_NAME": "Olievenhoutbos",
                    "MN_MDB_C": "TSH",
                    "MN_CODE": 799,
                    "MN_CODE_st": "799",
                    "MN_NAME": "City of Tshwane",
                    "DC_MDB_C": "TSH",
                    "DC_MN_C": 799,
                    "DC_MN_C_st": "799",
                    "DC_NAME": "City of Tshwane",
                    "PR_MDB_C": "GT",
                    "PR_CODE": 7,
                    "PR_CODE_st": "7",
                    "PR_NAME": "Gauteng",
                    "ALBERS_ARE": 0.143416,
                    "Shape_Leng": 0.015471,
                    "Shape_Area": 0.000013
                },
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [
                        [
                            [ 28.094077, -25.908263 ],
                            [ 28.093557, -25.908198 ],
                            [ 28.090423, -25.907788 ],
                            [ 28.094077, -25.908263 ]
                        ]
                    ]
                }
            }, {
                "type": "Feature",
                "id": 2,
                "properties": {
                    "SP_CODE": 799078004,
                    "SP_CODE_st": "799078004",
                    "SP_NAME": "NEWLANDS",
                    "MP_CODE": 799078,
                    "MP_CODE_st": "799078",
                    "MP_NAME": "Olievenhoutbos",
                    "MN_MDB_C": "TSH",
                    "MN_CODE": 799,
                    "MN_CODE_st": "799",
                    "MN_NAME": "City of Tshwane",
                    "DC_MDB_C": "TSH",
                    "DC_MN_C": 799,
                    "DC_MN_C_st": "799",
                    "DC_NAME": "City of Tshwane",
                    "PR_MDB_C": "GT",
                    "PR_CODE": 7,
                    "PR_CODE_st": "7",
                    "PR_NAME": "Gauteng",
                    "ALBERS_ARE": 0.59043,
                    "Shape_Leng": 0.038927,
                    "Shape_Area": 0.000053
                },
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [
                        [
                            [ 28.096909, -25.905686 ],
                            [ 28.096212, -25.90543 ],
                            [ 28.093557, -25.908198 ],
                            [ 28.096909, -25.905686 ]
                        ]
                    ]
                }
            }, {
                "type": "Feature",
                "id": 3,
                "properties": {
                    "SP_CODE": 799078005,
                    "SP_CODE_st": "799078005",
                    "SP_NAME": "SOSHANGUVE EAST",
                    "MP_CODE": 799078,
                    "MP_CODE_st": "799078",
                    "MP_NAME": "Olievenhoutbos",
                    "MN_MDB_C": "TSH",
                    "MN_CODE": 799,
                    "MN_CODE_st": "799",
                    "MN_NAME": "City of Tshwane",
                    "DC_MDB_C": "TSH",
                    "DC_MN_C": 799,
                    "DC_MN_C_st": "799",
                    "DC_NAME": "City of Tshwane",
                    "PR_MDB_C": "GT",
                    "PR_CODE": 7,
                    "PR_CODE_st": "7",
                    "PR_NAME": "Gauteng",
                    "ALBERS_ARE": 5.111315,
                    "Shape_Leng": 0.11507,
                    "Shape_Area": 0.00046
                },
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [
                        [
                            [ 28.120268, -25.900284 ],
                            [ 28.120077, -25.89996 ],
                            [ 28.119871, -25.899611 ],
                            [ 28.117086, -25.9021 ],
                            [ 28.117425, -25.902778 ],
                            [ 28.117464, -25.902857 ],
                            [ 28.120268, -25.900284 ]
                        ]
                    ]
                }
            }, {
                "type": "Feature",
                "id": 4,
                "properties": {
                    "SP_CODE": 799078006,
                    "SP_CODE_st": "799078006",
                    "SP_NAME": "MAGALIESKRUIN",
                    "MP_CODE": 799078,
                    "MP_CODE_st": "799078",
                    "MP_NAME": "Olievenhoutbos",
                    "MN_MDB_C": "TSH",
                    "MN_CODE": 799,
                    "MN_CODE_st": "799",
                    "MN_NAME": "City of Tshwane",
                    "DC_MDB_C": "TSH",
                    "DC_MN_C": 799,
                    "DC_MN_C_st": "799",
                    "DC_NAME": "City of Tshwane",
                    "PR_MDB_C": "GT",
                    "PR_CODE": 7,
                    "PR_CODE_st": "7",
                    "PR_NAME": "Gauteng",
                    "ALBERS_ARE": 0.414983,
                    "Shape_Leng": 0.030855,
                    "Shape_Area": 0.000037
                },
                "geometry": {
                    "type": "Polygon",
                    "coordinates": [
                        [
                            [ 28.098344, -25.917966 ],
                            [ 28.097966, -25.917912 ],
                            [ 28.097348, -25.918184 ],
                            [ 28.09721, -25.918154 ],
                            [ 28.098344, -25.917966 ]
                        ]
                    ]
                }
            }
        ]
    }
}"#;
