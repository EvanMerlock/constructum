use std::{fmt::Display, error::Error};

use axum::{response::{IntoResponse, Response}, http::{StatusCode}};


#[derive(Debug)]
pub enum ConstructumWebhookError {
    IOError(std::io::Error),
    YAMLDeserializeError(serde_yaml::Error),
}

impl Display for ConstructumWebhookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstructumWebhookError::IOError(io) => write!(f, "Webhook: IO Error: {io}"),
            ConstructumWebhookError::YAMLDeserializeError(yaml) => write!(f, "Webhook: YAML Deserialize Error: {yaml}"),
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

impl From<std::io::Error> for ConstructumWebhookError {
    fn from(value: std::io::Error) -> Self {
        ConstructumWebhookError::IOError(value)
    }
}

impl From<serde_yaml::Error> for ConstructumWebhookError {
    fn from(value: serde_yaml::Error) -> Self {
        ConstructumWebhookError::YAMLDeserializeError(value)
    }
}