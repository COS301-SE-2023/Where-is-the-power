use crate::{
    api::{ApiError, ApiResponse},
    auth::JWTAuthToken,
    db::Entity,
    user::User,
    DB_NAME,
};
use bson::oid::ObjectId;
use macros::Entity;
use mongodb::Client;
use rocket::{get, post, serde::json::Json, State};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use utoipa::ToSchema;

#[utoipa::path(post, path = "/api/reports", request_body = NewUserReport, security(("jwt" = [])))]
#[post("/reports", format = "application/json", data = "<new_report>")]
pub async fn create_report<'a>(
    token: JWTAuthToken,
    new_report: Json<NewUserReport>,
    state: &State<Option<Client>>,
) -> ApiResponse<'a, &'a str> {
    if token.email.is_none() {
        return ApiError::AuthError("Registered user required to submit report").into();
    }

    let db = state
        .as_ref()
        .expect("This rocket instance has no valid database!")
        .database(DB_NAME);

    let report = if let Some(user) = User::find_one(
        bson::doc! {
            "email": &token.email
        },
        &db,
        None,
    )
    .await
    {
        new_report.into_inner().into_entity(user.email.clone())
    } else {
        return ApiError::ServerError("Couldn't find the user associated with this token").into();
    };

    match report.insert(&db).await {
        Ok(_) => ApiResponse::Ok("Report created"),
        Err(err) => {
            log::error!("Couldn't insert report: {err:?}");
            ApiError::ServerError("Couldn't create report").into()
        }
    }
}

#[utoipa::path(get, path = "/api/reports")]
#[get("/reports")]
pub async fn get_reports(state: &State<Option<Client>>) -> ApiResponse<Vec<UserReportResponse>> {
    let db = state
        .as_ref()
        .expect("No attached mongodb client")
        .database(DB_NAME);

    match UserReport::find(bson::doc! {}, &db, None).await {
        Ok(reports) => ApiResponse::Ok(
            reports
                .into_iter()
                .map(|x| UserReportResponse::from(x.as_ref()))
                .filter(|x| x.expired == false)
                .collect(),
        ),
        Err(err) => {
            log::error!("Couldn't fetch user reports: {err:?}");
            ApiError::ServerError("Couldn't fetch user reports!").into()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[non_exhaustive]
pub enum ReportType {
    StolenCables,
    BlownSubstation,
    PowerOutage,
    DamagedCables,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[schema(example = json! {
    UserReportResponse {
        report_type: ReportType::StolenCables,
        email: "joe".to_string(),
        expired: false
    }
})]
pub struct UserReportResponse {
    pub report_type: ReportType,
    pub email: String,
    pub expired: bool,
}

impl From<&UserReport> for UserReportResponse {
    fn from(value: &UserReport) -> Self {
        Self {
            expired: value.is_expired(),
            report_type: value.report_type.clone(),
            email: value.email.clone(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Entity)]
#[collection_name = "reports"]
pub struct UserReport {
    #[serde(skip_serializing_if = "Option::is_none", rename = "_id")]
    pub id: Option<ObjectId>,
    pub report_type: ReportType,
    pub email: String,
    pub expires: u64,
}

impl UserReport {
    pub fn is_expired(&self) -> bool {
        u64::try_from(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        )
        .unwrap()
            > self.expires
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(example = json! {
    NewUserReport {
        report_type: ReportType::StolenCables,
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    }
})]
pub struct NewUserReport {
    pub report_type: ReportType,
    pub timestamp: u128,
}

impl NewUserReport {
    pub fn into_entity(self, email: String) -> UserReport {
        UserReport {
            id: None,
            report_type: self.report_type,
            expires: u64::try_from(self.timestamp).unwrap() + 1000 * 60 * 30,
            email,
        }
    }
}
