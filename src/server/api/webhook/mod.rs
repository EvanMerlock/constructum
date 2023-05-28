use axum::{Json, extract::State, routing::post};
use serde::{Deserialize, Serialize};

use uuid::Uuid;

use crate::{ConstructumState, server::{self, CreateJobPayload}};

use self::{error::ConstructumWebhookError, payload::GitWebhookPayload};

pub mod error;
pub mod payload;

#[axum_macros::debug_handler]
pub async fn webhook(
    State(state): State<ConstructumState>,
    Json(payload): Json<GitWebhookPayload>,
) -> axum::response::Result<Json<WebhookResult>, ConstructumWebhookError> {
    let create_job_payload = CreateJobPayload::new(payload.repository.id, payload.repository.html_url, payload.repository.name, payload.after);
    let pipeline_uuid = server::create_job(create_job_payload, state).await?;

    Ok(Json(WebhookResult {
        job_uuid: pipeline_uuid
    }))
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookResult {
    job_uuid: Uuid,
}

pub fn register_module(router: axum::Router<ConstructumState, axum::body::Body>) -> axum::Router<ConstructumState, axum::body::Body> {
    router
        .route("/webhook", post(webhook))
}