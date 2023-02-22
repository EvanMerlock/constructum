use std::{fmt::Display, error::Error};

use axum::{response::{IntoResponse, Response}, http::{StatusCode}};

use crate::git;


#[derive(Debug)]
pub enum ConstructumWebhookError {
    IOError(std::io::Error),
    YAMLDeserializeError(serde_yaml::Error),
    JSONSerializeError(serde_json::Error),
    GitError(git::GitError),
    KubeError(kube::Error),
    SqlError(sqlx::Error),
}

impl Display for ConstructumWebhookError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstructumWebhookError::IOError(io) => write!(f, "Webhook: IO Error: {io}"),
            ConstructumWebhookError::YAMLDeserializeError(yaml) => write!(f, "Webhook: YAML Deserialize Error: {yaml}"),
            ConstructumWebhookError::GitError(git) => write!(f, "Webhook: Git Error: {git}"),
            ConstructumWebhookError::KubeError(kube) => write!(f, "Webhook: Kube Error: {kube}"),
            ConstructumWebhookError::JSONSerializeError(json) => write!(f, "Webhook: JSON Serialize Error: {json}"),
            ConstructumWebhookError::SqlError(sql) => write!(f, "Webhook: SQL Error: {sql}"),
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

impl From<git::GitError> for ConstructumWebhookError {
    fn from(value: git::GitError) -> Self {
        ConstructumWebhookError::GitError(value)
    }
}

impl From<kube::Error> for ConstructumWebhookError {
    fn from(value: kube::Error) -> Self {
        ConstructumWebhookError::KubeError(value)
    }
}

impl From<serde_json::Error> for ConstructumWebhookError {
    fn from(value: serde_json::Error) -> Self {
        ConstructumWebhookError::JSONSerializeError(value)
    }
}

impl From<sqlx::Error> for ConstructumWebhookError {
    fn from(value: sqlx::Error) -> Self {
        ConstructumWebhookError::SqlError(value)
    }
}