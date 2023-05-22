use log::LevelFilter;
use rocket::{get, routes, Build, Rocket};
use std::time::SystemTime;
mod scraper;
mod scrapers;

#[cfg(test)]
mod tests;

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
    rocket::custom(figment).mount("/hello", routes![hi])
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    setup_logger().expect("Couldn't setup logger!");
    build_rocket().launch().await?;

    Ok(())
}
