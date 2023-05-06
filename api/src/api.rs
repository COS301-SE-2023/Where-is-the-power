#![allow(dead_code)]

use serde::Serialize;

#[derive(Serialize)]
#[non_exhaustive]
pub enum ApiError<'a> {
    AuthError(&'a str),
}
