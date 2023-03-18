use std::{fmt::Display, error::Error};

use axum::{response::{IntoResponse, Response}, http::{StatusCode}};

use crate::{server::error::ConstructumServerError};


#[derive(Debug)]
pub enum ConstructumWebhookError {
    ServerError(ConstructumServerError),
}

impl Display for ConstructumWebhookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstructumWebhookError::ServerError(server_e) => write!(f, "Webhook: {server_e}"),
        }
    }
}

impl Error for ConstructumWebhookError {}

impl IntoResponse for ConstructumWebhookError {
    fn into_response(self) -> Response {
        let resp_body = format!("{self}");

        (StatusCode::INTERNAL_SERVER_ERROR, resp_body).into_response()
    }
}

impl From<ConstructumServerError> for ConstructumWebhookError {
    fn from(value: ConstructumServerError) -> Self {
        ConstructumWebhookError::ServerError(value)
    }
}