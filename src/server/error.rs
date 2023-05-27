use std::{error::Error, fmt::Display};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::git;

#[derive(Debug)]
pub enum ConstructumServerError {
    IO(std::io::Error),
    YAMLDeserialize(serde_yaml::Error),
    JSONSerialize(serde_json::Error),
    Git(git::GitError),
    Kubernetes(kube::Error),
    Sql(sqlx::Error),
    HeaderToStrError(reqwest::header::ToStrError),
    BadAuthorization,
    ReqwestError(reqwest::Error),
    NoRepoFound,
    RepoAlreadyRegistered,
}

impl Display for ConstructumServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstructumServerError::IO(io) => write!(f, "Server: IO Error: {io}"),
            ConstructumServerError::YAMLDeserialize(yaml) => {
                write!(f, "Server: YAML Deserialize Error: {yaml}")
            }
            ConstructumServerError::Git(git) => write!(f, "Server: Git Error: {git}"),
            ConstructumServerError::Kubernetes(kube) => write!(f, "Server: Kube Error: {kube}"),
            ConstructumServerError::JSONSerialize(json) => {
                write!(f, "Server: JSON Serialize Error: {json}")
            }
            ConstructumServerError::Sql(sql) => write!(f, "Server: SQL Error: {sql}"),
            ConstructumServerError::HeaderToStrError(tostr) => write!(
                f,
                "Server: Failed to deserialized header from string: {tostr}"
            ),
            ConstructumServerError::BadAuthorization => {
                write!(f, "Server: Bad authorization token sent in")
            }
            ConstructumServerError::ReqwestError(reqe) => write!(f, "Server: Reqwest error {reqe}"),
            ConstructumServerError::NoRepoFound => write!(f, "Server: Repo Not Found"),
            ConstructumServerError::RepoAlreadyRegistered => {
                write!(f, "Server: Repo Already Registered")
            }
        }
    }
}

impl Error for ConstructumServerError {}

impl IntoResponse for ConstructumServerError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct Error {
            error: String,
        }

        let resp_body = format!("{self}");

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            serde_json::to_string(&Error { error: resp_body })
                .expect("failed to construct serde response"),
        )
            .into_response()
    }
}

impl From<std::io::Error> for ConstructumServerError {
    fn from(value: std::io::Error) -> Self {
        ConstructumServerError::IO(value)
    }
}

impl From<serde_yaml::Error> for ConstructumServerError {
    fn from(value: serde_yaml::Error) -> Self {
        ConstructumServerError::YAMLDeserialize(value)
    }
}

impl From<git::GitError> for ConstructumServerError {
    fn from(value: git::GitError) -> Self {
        ConstructumServerError::Git(value)
    }
}

impl From<kube::Error> for ConstructumServerError {
    fn from(value: kube::Error) -> Self {
        ConstructumServerError::Kubernetes(value)
    }
}

impl From<serde_json::Error> for ConstructumServerError {
    fn from(value: serde_json::Error) -> Self {
        ConstructumServerError::JSONSerialize(value)
    }
}

impl From<sqlx::Error> for ConstructumServerError {
    fn from(value: sqlx::Error) -> Self {
        ConstructumServerError::Sql(value)
    }
}

impl From<reqwest::header::ToStrError> for ConstructumServerError {
    fn from(value: reqwest::header::ToStrError) -> Self {
        ConstructumServerError::HeaderToStrError(value)
    }
}

impl From<reqwest::Error> for ConstructumServerError {
    fn from(value: reqwest::Error) -> Self {
        ConstructumServerError::ReqwestError(value)
    }
}
