mod api;
mod auth;
#[cfg(test)]
mod tests;
mod user;

use api::ApiError;
use auth::{AuthRequest, AuthResponder, AuthType, JWTAuthToken};
use log::LevelFilter;
use rocket::serde::json::Json;
use rocket::{get, post, routes, Build, Rocket};
use std::time::SystemTime;

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

fn build_rocket() -> Rocket<Build> {
    let figment = rocket::Config::figment();
    rocket::custom(figment)
        .mount("/hello", routes![hi])
        .mount("/api", routes![authenticate])
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    setup_logger().expect("Couldn't setup logger!");

    build_rocket().launch().await?;

    Ok(())
}
