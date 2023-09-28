mod ai;
mod api;
mod auth;
mod db;
mod dns;
mod loadshedding;
mod reporting;
mod scraper;
#[cfg(test)]
mod tests;
mod user;

use crate::scraper::UploadRequest;
use api::ApiError;

use bson::doc;
use loadshedding::StageUpdater;
use log::{info, warn, LevelFilter};
use mongodb::options::ClientOptions;
use mongodb::Client;
use rocket::config::TlsConfig;
use rocket::data::{Limits, ToByteUnit};
use rocket::figment::Figment;
use rocket::fs::FileServer;

use rocket::http::Method;
use rocket::serde::json::Json;
use rocket::{post, routes, Build, Rocket, State};
use rocket_cors::{AllowedHeaders, CorsOptions};
use std::env;
use std::net::IpAddr;
use std::time::SystemTime;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

const DB_NAME: &'static str = "wip";

#[derive(OpenApi)]
#[openapi(
    paths(
        user::create_user,
        loadshedding::get_current_stage,
        loadshedding::fetch_map_data,
        loadshedding::fetch_schedule,
        loadshedding::fetch_suburb_stats,
        loadshedding::fetch_time_for_polygon,
        auth::authenticate,
        ai::get_ai_info,
        user::get_saved_places,
        user::add_saved_place,
        user::delete_saved_place,
        reporting::create_report,
        reporting::get_reports
    ),
    components(schemas(
        auth::AuthRequest,
        auth::AuthType,
        user::NewUser,
        user::UserLocation,
        loadshedding::MapDataRequest,
        loadshedding::MapDataDefaultResponse,
        loadshedding::PredictiveSuburbStatsResponse,
        loadshedding::SuburbStatsRequest,
        api::ResponseString,
        api::ApiError,
        ai::AiInfoRequest,
        user::SavedPlace,
        reporting::NewUserReport,
        reporting::ReportType
    )),
    info(title = "Where Is The Power API Specification"),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "jwt",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        )
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

#[cfg(debug_assertions)]
const LOG_LEVEL: LevelFilter = LevelFilter::Debug;
#[cfg(not(debug_assertions))]
const LOG_LEVEL: LevelFilter = LevelFilter::Info;

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

async fn get_config() -> Figment {
    let mut figment =
        rocket::Config::figment().merge(("limits", Limits::new().limit("json", 7.megabytes())));

    let ssl_cert = if !tokio::fs::try_exists("ssl/ssl_cert.pem")
        .await
        .unwrap_or(false)
    {
        warn!("Didn't find TLS certificate, checking environment vars");
        if let Ok(ssl_cert) = env::var("TLS_CERT") {
            Some(ssl_cert)
        } else {
            None
        }
    } else {
        info!("Found TLS cert, reading...");
        tokio::fs::read_to_string("ssl/ssl_cert.pem").await.ok()
    };

    let ssl_key = if !tokio::fs::try_exists("ssl/ssl_private_key.pem")
        .await
        .unwrap_or(false)
    {
        warn!("Didn't find TLS private key, checking environment vars");
        if let Ok(ssl_key) = env::var("TLS_KEY") {
            Some(ssl_key)
        } else {
            warn!("Couldn't find TLS private key");
            None
        }
    } else {
        info!("Found TLS private key, reading...");
        tokio::fs::read_to_string("ssl/ssl_private_key.pem")
            .await
            .ok()
    };

    if ssl_cert.is_some() && ssl_key.is_some() {
        let tls_cfg =
            TlsConfig::from_bytes(ssl_cert.unwrap().as_bytes(), ssl_key.unwrap().as_bytes());
        figment = figment.merge(("tls", tls_cfg));
    } else {
        warn!("Couldn't find TLS keys, not setting up TLS");
    }

    figment
}

async fn build_rocket() -> Rocket<Build> {
    if let Err(err) = dotenvy::dotenv() {
        warn!("Couldn't read .env file! {err:?}");
    }

    let figment = get_config().await;
    let db_uri = env::var("DATABASE_URI").unwrap_or(String::from(""));
    // Cors Options, we should modify to our needs but leave as default for now.
    let cors = CorsOptions {
        allowed_origins: rocket_cors::AllOrSome::All,
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Patch,
            Method::Options,
            Method::Head,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    let rocket_no_state = || {
        rocket::custom(figment.clone())
            .mount(
                "/",
                SwaggerUi::new("/swagger-ui/<_..>")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
            .mount(
                "/api",
                routes!(
                    auth::authenticate,
                    user::create_user,
                    loadshedding::get_current_stage,
                    loadshedding::fetch_map_data,
                    loadshedding::fetch_suburb_stats,
                    loadshedding::fetch_schedule,
                    loadshedding::fetch_time_for_polygon,
                    user::add_saved_place,
                    user::get_saved_places,
                    ai::get_ai_info,
                    user::delete_saved_place,
                    reporting::create_report,
                    reporting::get_reports
                ),
            )
            .mount("/upload", routes![upload_data])
            .mount(
                "/api-docs",
                FileServer::new("api-docs", rocket::fs::Options::IndexFile),
            )
            .attach(StageUpdater)
            .attach(cors.clone())
            .manage::<Option<Client>>(None)
    };

    match ClientOptions::parse(&db_uri).await {
        Ok(client_options) => match Client::with_options(client_options) {
            Ok(client) => rocket::custom(figment.clone())
                .mount(
                    "/",
                    SwaggerUi::new("/swagger-ui/<_..>")
                        .url("/api-docs/openapi.json", ApiDoc::openapi()),
                )
                .mount(
                    "/api",
                    routes!(
                        auth::authenticate,
                        user::create_user,
                        loadshedding::get_current_stage,
                        loadshedding::fetch_map_data,
                        loadshedding::fetch_suburb_stats,
                        loadshedding::fetch_schedule,
                        loadshedding::fetch_time_for_polygon,
                        user::add_saved_place,
                        user::get_saved_places,
                        ai::get_ai_info,
                        user::delete_saved_place,
                        reporting::create_report,
                        reporting::get_reports
                    ),
                )
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

    if let Err(err) = dns::update_dns().await {
        warn!("Couldn't setup DNS: {err:?}");
    }

    build_rocket().await.launch().await?;
    Ok(())
}
