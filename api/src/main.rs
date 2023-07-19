mod api;
mod auth;
mod db;
mod dns;
mod loadshedding;
mod scraper;
#[cfg(test)]
mod tests;
mod user;

use crate::scraper::UploadRequest;
use api::ApiError;
use auth::{AuthRequest, AuthResponder, AuthType, JWTAuthToken};
use bson::doc;
use db::Entity;
use loadshedding::{
    LoadSheddingStage, MapDataDefaultResponse, MapDataRequest, MunicipalityEntity, StageUpdater,
};
use log::{warn, LevelFilter};
use mongodb::options::{ClientOptions, FindOptions};
use mongodb::{Client, Cursor};
use rocket::data::{Limits, ToByteUnit};
use rocket::futures::future::try_join_all;
use rocket::futures::TryStreamExt;
use rocket::http::Method;
use rocket_cors::{CorsOptions, AllowedHeaders};
use rocket::serde::json::Json;
use rocket::{get, post, routes, Build,Rocket, State};
use std::env;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::SystemTime;
use tokio::sync::RwLock;
use user::User;

#[post("/fetchMapData", format = "application/json", data = "<request>")]
async fn fetch_map_data(
    db: &State<Option<Client>>,
    loadshedding_stage: &State<Option<Arc<RwLock<LoadSheddingStage>>>>,
    request: Json<MapDataRequest>,
) -> Result<Json<MapDataDefaultResponse>, Json<ApiError<'static>>> {
    let connection = &db.inner().as_ref().unwrap().database("staging");
    let south_west: Vec<f64> = request.bottom_left.iter().cloned().map(|x| x).collect();
    let north_east: Vec<f64> = request.top_right.iter().cloned().map(|x| x).collect();
    let query = doc! {
        "geometry.bounds" : {
            "$geoWithin" : {
                "$box" : [south_west, north_east]
            }
        }
    };
    let options = FindOptions::default();
    let cursor: Cursor<MunicipalityEntity> = match connection
        .collection("municipality")
        .find(query, options)
        .await
    {
        Ok(cursor) => cursor,
        Err(err) => {
            log::error!("Database error occured when handling geo query: {err}");
            return Err(Json(ApiError::ServerError(
                "Database error occured when handling request. Check logs.",
            )));
        }
    };
    let stage = &loadshedding_stage
        .inner()
        .as_ref()
        .clone()
        .unwrap()
        .read()
        .await
        .stage;
    let municipalities: Vec<MunicipalityEntity> = match cursor.try_collect().await {
        Ok(item) => item,
        Err(err) => {
            log::error!("Unable to Collect suburbs from cursor {err}");
            return Err(Json(ApiError::ServerError(
                "Error occured on the server, sorry :<",
            )));
        }
    };
    let future_data = municipalities.iter().map(|municipality| {
        municipality.get_regions_at_time(stage.to_owned(), request.time, connection)
    });
    let response = try_join_all(future_data).await;
    if let Ok(data) = response {
        return Ok(Json(data.into_iter().fold(
            MapDataDefaultResponse {
                map_polygons: vec![],
                on: vec![],
                off: vec![],
            },
            |acc, obj| acc + obj,
        )));
    } else {
        log::error!("Unable to fold MapDataResponse");
        return Err(Json(ApiError::ServerError(
            "Error occured on the server, sorry :<",
        )));
    }
}

#[post("/uploadData", format = "application/json", data = "<upload_data>")]
async fn upload_data(
    state: &State<Option<Client>>,
    upload_data: Json<UploadRequest>,
    ip: IpAddr,
) -> Result<&'static str, Json<ApiError<'static>>> {
    if !ip.is_loopback() {
        return Ok("304 you do not have access to this resource");
    }
    if state.is_none() {
        return Err(Json(ApiError::ServerError(
            "Database is unavailable. Please try again later!",
        )));
    }
    let data = upload_data.into_inner();
    // Process the data and return an appropriate response
    // validate
    let add_data = data
        .add_data(&state.inner().as_ref().unwrap(), "staging")
        .await;
    match add_data {
        Ok(()) => return Ok("Data Successfully added to staging database and ready for review"),
        Err(e) => return Err(e),
    }
}

#[post("/auth", format = "application/json", data = "<auth_request>")]
async fn authenticate(
    auth_request: Json<AuthRequest>,
) -> Result<AuthResponder, Json<ApiError<'static>>> {
    match auth_request.auth_type {
        AuthType::Anonymous => Ok(AuthResponder {
            inner: Json(JWTAuthToken::new(auth_request.auth_type).unwrap()),
            header: rocket::http::Header::new(
                "Set-Cookie",
                "cookie=some_cookie;expires=0;path=/;SameSite=Strict".to_string(),
            ),
        }),
        AuthType::User => unimplemented!(),
    }
}

#[post("/user", format = "application/json", data = "<new_user>")]
async fn create_user(
    state: &State<Option<Client>>,
    new_user: Json<User>,
) -> Result<&'static str, Json<ApiError<'static>>> {
    if state.is_none() {
        return Err(Json(ApiError::ServerError(
            "Database is unavailable. Please try again later!",
        )));
    }

    let state = state.inner().as_ref().unwrap();

    new_user
        .insert(&state.database("wip"))
        .await
        .expect("Couldn't inser new user!");

    Ok("User created")
}

#[get("/world")]
async fn hi() -> &'static str {
    "Hello World!"
}

#[cfg(debug_assertions)]
const LOG_LEVEL: LevelFilter = LevelFilter::Debug;
#[cfg(not(debug_assertions))]
const LOG_LEVEL: LevelFilter = LevelFilter::Warn;

fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339_seconds(SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(LOG_LEVEL)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

async fn build_rocket() -> Rocket<Build> {
    let figment =
        rocket::Config::figment().merge(("limits", Limits::new().limit("json", 7.megabytes())));

    let db_uri = env::var("DATABASE_URI").unwrap_or(String::from(""));
    // Cors Options, we should modify to our needs but leave as default for now.
    let cors = CorsOptions {
        allowed_origins : rocket_cors::AllOrSome::All,
        allowed_methods : vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();

    let rocket_no_state = || {
        rocket::custom(figment.clone())
            .mount("/hello", routes![hi])
            .mount("/api", routes!(authenticate, create_user, fetch_map_data))
            .mount("/upload", routes![upload_data])
            .attach(StageUpdater)
            .attach(cors.clone())
            .manage::<Option<Client>>(None)
    };

    match ClientOptions::parse(&db_uri).await {
        Ok(client_options) => match Client::with_options(client_options) {
            Ok(client) => rocket::custom(figment.clone())
                .mount("/hello", routes![hi])
                .mount("/api", routes!(authenticate, create_user, fetch_map_data))
                .mount("/upload", routes![upload_data])
                .attach(StageUpdater)
                .attach(cors)
                .manage(Some(client)),
            Err(err) => {
                warn!("Couldn't create database client! {err:?}");
                rocket_no_state()
            }
        },
        Err(err) => {
            warn!("Couldn't create database config! {err:?}");
            rocket_no_state()
        }
    }
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    setup_logger().expect("Couldn't setup logger!");

    if let Err(err) = dotenvy::dotenv() {
        warn!("Couldn't read .env file! {err:?}");
    }
    if let Err(err) = dns::update_dns().await {
        warn!("Couldn't setup DNS: {err:?}");
    }

    build_rocket().await.launch().await?;
    Ok(())
}
