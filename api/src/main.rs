use log::LevelFilter;
use rocket::{get, routes, Build, Rocket};
use std::time::SystemTime;
mod scraper;
mod scrapers;

mod api;
mod auth;
mod db;
#[cfg(test)]
mod tests;
mod user;

use api::ApiError;
use auth::{AuthRequest, AuthResponder, AuthType, JWTAuthToken};
use db::Entity;
use log::LevelFilter;
use mongodb::options::ClientOptions;
use mongodb::Client;
use rocket::serde::json::Json;
use rocket::{get, post, routes, Build, Rocket, State};
use std::env;
use std::time::SystemTime;
use user::User;

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
    state: &State<Client>,
    new_user: Json<User>,
) -> Result<&'static str, Json<ApiError<'static>>> {
    new_user
        .insert(&state.inner().database("wip"))
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
    dotenvy::dotenv().expect("Couldn't load .env file");
    let db_uri = env::var("DATABASE_URI").expect("DATABASE_URI must be set in .env file");
    let client_options = ClientOptions::parse(&db_uri)
        .await
        .expect("Couldn't setup database client configuration");

    let db_client = Client::with_options(client_options).expect("Couldn't connect to database");

    let figment = rocket::Config::figment();
    rocket::custom(figment)
        .mount("/hello", routes![hi])
        .mount("/api", routes![authenticate, create_user])
        .manage(db_client)
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    setup_logger().expect("Couldn't setup logger!");

    build_rocket().await.launch().await?;

    Ok(())
}
