use std::{error::Error, fmt::Display};

use axum::{response::IntoResponse, http::StatusCode, Json};
use serde_json::json;


#[derive(Debug)]
pub enum ConstructumServerError {
    SqlError(sqlx::Error),
}

impl IntoResponse for ConstructumServerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ConstructumServerError::SqlError(sqle) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("{sqle}") })))
            },
        }.into_response()
    }
}

impl Display for ConstructumServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstructumServerError::SqlError(sqle) => write!(f, "ConstructumServerError: SQL Error: {sqle}"),
        }
    }
}

impl Error for ConstructumServerError {}

impl From<sqlx::Error> for ConstructumServerError {
    fn from(value: sqlx::Error) -> Self {
        Self::SqlError(value)
    }
}