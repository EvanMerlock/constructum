use axum::{Json, response::IntoResponse, http::StatusCode};
use serde::{Deserialize, Serialize};

pub mod webhook;

pub async fn root() -> &'static str {
    "Constructum Root"
}