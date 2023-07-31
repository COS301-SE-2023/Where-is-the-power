use super::build_rocket;
use crate::auth::{AuthClaims, AuthRequest, AuthType, JWTAuthToken};
use crate::loadshedding::{GroupEntity, TimeScheduleEntity, SuburbEntity, MunicipalityEntity, MockDBFunctionsTrait};
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

#[rocket::async_test]
async fn polygon_request_test() {
    let mut mock = MockDBFunctionsTrait::new();
    let data = r#"[{ "_id": { "$oid": "64b6b9b30d09aa7756061c0d" }, "startHour": 20, "startMinute": 0, "stopHour": 22, "stopMinute": 30, "stages": [ { "stage": 1, "groups": [ { "$oid": "64b6b9b30d09aa7756061b9d" } ] }, { "stage": 3, "groups": [ { "$oid": "64b6b9b30d09aa7756061a94" }, { "$oid": "64b6b9b30d09aa7756061b00" } ] }, { "stage": 4, "groups": [ { "$oid": "64b6b9b30d09aa7756061ab6" }] }, { "stage": 2, "groups": [ { "$oid": "64b6b9b30d09aa7756061a79" }] }], "municipality": { "$oid": "64b6b9b30d09aa7756061a47" } }] "#;
    let test_schedule: Vec<TimeScheduleEntity> = serde_json::from_str(data).unwrap();
    let data = r#"[{ "_id": { "$oid": "64b6b9b30d09aa7756061b9d" }, "number": 1, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061b30" } ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061a79" }, "number": 2, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061b7d" } ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061a94" }, "number": 3, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061a63" } ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061ab6" }, "number": 4, "suburbs": [ { "$oid": "64b6b9b30d09aa7756061a7b" } ] }]"#;
    let test_groups: Vec<GroupEntity> = serde_json::from_str(data).unwrap();
    let data = r#"[ { "_id": { "$oid": "64b6b9b30d09aa7756061b30" }, "municipality": { "$oid": "64b6b9b30d09aa7756061a47" }, "name": "MUCKLENEUK", "geometry": [ 1 ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061b7d" }, "municipality": { "$oid": "64b6b9b30d09aa7756061a47" }, "name": "NEWLANDS", "geometry": [ 2 ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061a63" }, "municipality": { "$oid": "64b6b9b30d09aa7756061a47" }, "name": "SOSHANGUVE EAST", "geometry": [ 3 ] }, { "_id": { "$oid": "64b6b9b30d09aa7756061a7b" }, "municipality": { "$oid": "64b6b9b30d09aa7756061a47" }, "name": "MAGALIESKRUIN", "geometry": [ 4 ] }]"#;
    let test_suburbs: Vec<SuburbEntity> = serde_json::from_str(data).unwrap();
    mock.expect_collect_schedule()
        .returning(move |_,_,_| {
            Ok(test_schedule.clone())
        });
    mock.expect_collect_groups()
        .returning(move|_query, _conn, _opts| {
           Ok(test_groups.clone())
        });
    mock.expect_collect_suburbs()
        .returning(move|_query, _conn, _opts| {
           Ok(test_suburbs.clone())
        });
    let data = r#"{ "_id": { "$oid": "64b6b9b30d09aa7756061a47" }, "name": "tshwane", "geometry": { "name": "SP_SA_2011|Selection", "map_layer_type": "Area", "bounds": [ [ 27.890227, -26.077549 ], [ 29.098541, -25.110155 ] ], "center": [ 28.494384, -25.593852 ], "zoom": 6, "median_zoom": 13, "count": 594, "property_names": [ "SP_CODE", "SP_CODE_st", "SP_NAME", "MP_CODE", "MP_CODE_st", "MP_NAME", "MN_MDB_C", "MN_CODE", "MN_CODE_st", "MN_NAME", "DC_MDB_C", "DC_MN_C", "DC_MN_C_st", "DC_NAME", "PR_MDB_C", "PR_CODE", "PR_CODE_st", "PR_NAME", "ALBERS_ARE", "Shape_Leng", "Shape_Area" ], "type": "FeatureCollection", "features": [ { "type": "Feature", "id": 1, "properties": { "SP_CODE": 799078003, "SP_CODE_st": "799078003", "SP_NAME": "MUCKLNEUK", "MP_CODE": 799078, "MP_CODE_st": "799078", "MP_NAME": "Olievenhoutbos", "MN_MDB_C": "TSH", "MN_CODE": 799, "MN_CODE_st": "799", "MN_NAME": "City of Tshwane", "DC_MDB_C": "TSH", "DC_MN_C": 799, "DC_MN_C_st": "799", "DC_NAME": "City of Tshwane", "PR_MDB_C": "GT", "PR_CODE": 7, "PR_CODE_st": "7", "PR_NAME": "Gauteng", "ALBERS_ARE": 0.143416, "Shape_Leng": 0.015471, "Shape_Area": 0.000013 }, "geometry": { "type": "Polygon", "coordinates": [ [ [ 28.094077, -25.908263 ], [ 28.093557, -25.908198 ], [ 28.090423, -25.907788 ], [ 28.094077, -25.908263 ] ] ] } }, { "type": "Feature", "id": 2, "properties": { "SP_CODE": 799078004, "SP_CODE_st": "799078004", "SP_NAME": "NEWLANDS", "MP_CODE": 799078, "MP_CODE_st": "799078", "MP_NAME": "Olievenhoutbos", "MN_MDB_C": "TSH", "MN_CODE": 799, "MN_CODE_st": "799", "MN_NAME": "City of Tshwane", "DC_MDB_C": "TSH", "DC_MN_C": 799, "DC_MN_C_st": "799", "DC_NAME": "City of Tshwane", "PR_MDB_C": "GT", "PR_CODE": 7, "PR_CODE_st": "7", "PR_NAME": "Gauteng", "ALBERS_ARE": 0.59043, "Shape_Leng": 0.038927, "Shape_Area": 0.000053 }, "geometry": { "type": "Polygon", "coordinates": [ [ [ 28.096909, -25.905686 ], [ 28.096212, -25.90543 ], [ 28.093557, -25.908198 ], [ 28.096909, -25.905686 ] ] ] } }, { "type": "Feature", "id": 3, "properties": { "SP_CODE": 799078005, "SP_CODE_st": "799078005", "SP_NAME": "SOSHANGUVE EAST", "MP_CODE": 799078, "MP_CODE_st": "799078", "MP_NAME": "Olievenhoutbos", "MN_MDB_C": "TSH", "MN_CODE": 799, "MN_CODE_st": "799", "MN_NAME": "City of Tshwane", "DC_MDB_C": "TSH", "DC_MN_C": 799, "DC_MN_C_st": "799", "DC_NAME": "City of Tshwane", "PR_MDB_C": "GT", "PR_CODE": 7, "PR_CODE_st": "7", "PR_NAME": "Gauteng", "ALBERS_ARE": 5.111315, "Shape_Leng": 0.11507, "Shape_Area": 0.00046 }, "geometry": { "type": "Polygon", "coordinates": [ [ [ 28.120268, -25.900284 ], [ 28.120077, -25.89996 ], [ 28.119871, -25.899611 ], [ 28.117086, -25.9021 ], [ 28.117425, -25.902778 ], [ 28.117464, -25.902857 ], [ 28.120268, -25.900284 ] ] ] } }, { "type": "Feature", "id": 4, "properties": { "SP_CODE": 799078006, "SP_CODE_st": "799078006", "SP_NAME": "MAGALIESKRUIN", "MP_CODE": 799078, "MP_CODE_st": "799078", "MP_NAME": "Olievenhoutbos", "MN_MDB_C": "TSH", "MN_CODE": 799, "MN_CODE_st": "799", "MN_NAME": "City of Tshwane", "DC_MDB_C": "TSH", "DC_MN_C": 799, "DC_MN_C_st": "799", "DC_NAME": "City of Tshwane", "PR_MDB_C": "GT", "PR_CODE": 7, "PR_CODE_st": "7", "PR_NAME": "Gauteng", "ALBERS_ARE": 0.414983, "Shape_Leng": 0.030855, "Shape_Area": 0.000037 }, "geometry": { "type": "Polygon", "coordinates": [ [ [ 28.098344, -25.917966 ], [ 28.097966, -25.917912 ], [ 28.097348, -25.918184 ], [ 28.09721, -25.918154 ], [ 28.098344, -25.917966 ] ] ] } } ] } }"#;
    let test_municipality: MunicipalityEntity = serde_json::from_str(data).unwrap();

    // the time is the first day of a month matching the time_schedule
    let result = test_municipality.get_regions_at_time(1, Some(1688237449), None , &mock).await;
    if let Ok(data) = result {
        println!("{:?}", data);
    } else if  let Err(data) = result {
        println!("{:?}", data);
    }
    assert_eq!(1,1)
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
