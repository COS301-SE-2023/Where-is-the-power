use rocket::{post, serde::json::Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::api::ApiResponse;

#[utoipa::path(post, tag = "AI", path = "/api/ai/info", request_body = AiInfoRequest)]
#[post("/ai/info", format = "application/json", data = "<request>")]
async fn get_ai_info<'a>(request: Json<AiInfoRequest>) -> ApiResponse<'a, AiInfoResponse> {
    unimplemented!();
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

#[derive(Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AiInfoResponse {
    pub coords_to_avoid: Vec<[f64; 2]>,
}
