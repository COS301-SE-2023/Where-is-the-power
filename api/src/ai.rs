use crate::api::{ApiError, ApiResponse};
use rocket::{post, serde::json::Json};
use serde::{Deserialize, Serialize};
use std::env;
use std::process::Stdio;
use tokio::process::Command;
use utoipa::ToSchema;

#[utoipa::path(post, tag = "AI", path = "/api/ai/info", request_body = AiInfoRequest)]
#[post("/ai/info", format = "application/json", data = "<request>")]
pub async fn get_ai_info<'a>(request: Json<AiInfoRequest>) -> ApiResponse<'a, AiInfoResponse> {
    let mapbox_api_key = if let Ok(key) = dbg!(env::var("MAPBOX_API_KEY")) {
        key
    } else {
        log::error!("We couldn't get the mapbox api key. The environment variable was not set!");
        return ApiError::ServerError(
            "Invalid server configuration, please contact an administrator",
        )
        .into();
    };

    match Command::new("python3")
        .args([
            "route.py",
            serde_json::to_string(&request.into_inner())
                .unwrap()
                .as_ref(),
        ])
        .current_dir("src")
        .stdout(Stdio::piped())
        .env("MAPBOX_API_KEY", mapbox_api_key)
        .output()
        .await
    {
        Ok(val) => {
            match serde_json::from_str::<AiInfoResponse>(
                String::from_utf8_lossy(&val.stdout).as_ref(),
            ) {
                Ok(result) => ApiResponse::Ok(result),
                Err(err) => {
                    log::error!("Unable to parse python script output: {err:?}");
                    log::info!(
                        "Python script output: {}",
                        String::from_utf8_lossy(&val.stdout)
                    );
                    ApiError::ServerError("Unable to fetch AI map data at this time").into()
                }
            }
        }
        Err(err) => {
            log::error!("Couldn't run python script: {err:?}");
            ApiError::ServerError("Unable to fetch AI map data at this time").into()
        }
    }
}

#[derive(Clone, Deserialize, Serialize, ToSchema)]
#[schema(example = json! {
    AiInfoRequest {
        origin: Box::new([28.3, -27.73]),
        destination: Box::new([28.2651, -25.7597])
    }
})]
pub struct AiInfoRequest {
    pub origin: Box<[f64; 2]>,
    pub destination: Box<[f64; 2]>,
}

#[derive(Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AiInfoResponse {
    pub duration: f32,
    pub distance: f32,
    pub traffic_lights_avoided: Vec<[f32; 2]>,
    pub instructions: Vec<String>,
    pub coordinates: Vec<[f32; 2]>,
}
