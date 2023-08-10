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
            "src/avoid_cords.py",
            serde_json::to_string(&request.into_inner())
                .unwrap()
                .as_ref(),
        ])
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
        polygon: vec![
            [0.0, 0.0],
            [90.0, -90.0],
            [-90.0, 90.0],
        ]
    }
})]
pub struct AiInfoRequest {
    pub polygon: Vec<[f64; 2]>,
}

#[derive(Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AiInfoResponse {
    pub coords_to_avoid: Vec<[f64; 2]>,
}
