#![allow(dead_code)]

use crate::loadshedding::MapDataDefaultResponse;
use rocket::{http::ContentType, response::Responder};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema, Clone)]
#[non_exhaustive]
pub enum ApiError<'a> {
    AuthError(&'a str),
    UserCreationError(&'a str),
    SavedPlacesError(&'a str),
    ServerError(&'a str),
    ScraperUploadError(&'a str),
    RequestError(&'a str),
}

pub enum ApiResponse<'a, O: Serialize> {
    Ok(O),
    Err(ApiError<'a>),
}

#[derive(Serialize, ToSchema)]
#[schema(example = json! {
    UnifiedResponse::<()> {
        success: false,
        result: None,
        error: Some(ApiError::AuthError("Incorrect password"))
    }
})]
#[aliases(
    ResponseString = UnifiedResponse<'a, String>,
    ResponseMapData = UnifiedResponse<'a, MapDataDefaultResponse>
)]
pub struct UnifiedResponse<'b, O: Serialize> {
    success: bool,
    result: Option<O>,
    error: Option<ApiError<'b>>,
}

impl<'r, 'a, O: Serialize> Responder<'r, 'static> for ApiResponse<'a, O> {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let response = match self {
            Self::Ok(result) => UnifiedResponse {
                result: Some(result),
                success: true,
                error: None,
            },
            Self::Err(error) => UnifiedResponse {
                result: None,
                success: false,
                error: Some(error),
            },
        };

        let response = serde_json::to_string(&response).unwrap();
        rocket::Response::build()
            .header(ContentType::JSON)
            .sized_body(response.len(), std::io::Cursor::new(response))
            .ok()
    }
}

impl<'a, O: Serialize> From<ApiError<'a>> for ApiResponse<'a, O> {
    fn from(value: ApiError<'a>) -> Self {
        Self::Err(value)
    }
}
