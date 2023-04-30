use std::{error::Error, fmt::Display};

use crate::{config::ConstructumConfigError, git::GitError};

#[derive(Debug)]
pub enum ConstructumClientError {
    ConstructumConfigError(ConstructumConfigError),
    SqlError(sqlx::Error),
    IOError(tokio::io::Error),
    YamlDecodeError(serde_yaml::Error),
    PipelineExecError(PipelineExecError),
    GitError(GitError),
}

impl Display for ConstructumClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstructumClientError::ConstructumConfigError(cce) => write!(f, "Client Error: Config Error: {cce}"),
            ConstructumClientError::SqlError(sqle) => write!(f, "Client Error: SQL Error: {sqle}"),
            ConstructumClientError::IOError(iox) => write!(f, "Client Error: IO Error: {iox}"),
            ConstructumClientError::YamlDecodeError(yamle) => write!(f, "Client Error: Yaml Decode Error: {yamle}"),
            ConstructumClientError::PipelineExecError(pipee) => write!(f, "Client Error: Pipeline Error: {pipee}"),
            ConstructumClientError::GitError(gite) => write!(f, "Client Error: Config Error: {gite}"),
        }
    }
}

impl Error for ConstructumClientError {}

impl From<ConstructumConfigError> for ConstructumClientError {
    fn from(value: ConstructumConfigError) -> Self {
        ConstructumClientError::ConstructumConfigError(value)
    }
}

impl From<sqlx::Error> for ConstructumClientError {
    fn from(value: sqlx::Error) -> Self {
        ConstructumClientError::SqlError(value)
    }
}

impl From<tokio::io::Error> for ConstructumClientError {
    fn from(value: tokio::io::Error) -> Self {
        ConstructumClientError::IOError(value)
    }
}

impl From<serde_yaml::Error> for ConstructumClientError {
    fn from(value: serde_yaml::Error) -> Self {
        ConstructumClientError::YamlDecodeError(value)
    }
}

impl From<PipelineExecError> for ConstructumClientError {
    fn from(value: PipelineExecError) -> Self {
        ConstructumClientError::PipelineExecError(value)
    }
}

impl From<GitError> for ConstructumClientError {
    fn from(value: GitError) -> Self {
        ConstructumClientError::GitError(value)
    }
}

#[derive(Debug)]
pub enum PipelineExecError {
    KubernetesError(kube::Error),
    JsonEncodeError(serde_json::Error),
    KubeRuntimeWaitError(kube::runtime::wait::Error),
    SqlError(sqlx::Error),
    InvalidSecretConfiguration,
    ReqwestError(reqwest::Error),
    IOError(std::io::Error),
}

impl Display for PipelineExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelineExecError::KubernetesError(kerr) => write!(f, "Pipeline Error: Kube Error: {kerr}"),
            PipelineExecError::JsonEncodeError(jsone) => write!(f, "Pipeline Error: JSON Encode Error: {jsone}"),
            PipelineExecError::KubeRuntimeWaitError(krwe) => write!(f, "Pipeline Error: Kube Runtime Wait Error: {krwe}"),
            PipelineExecError::SqlError(sqle) => write!(f, "Pipeline Error: SQL Error: {sqle}"),
            PipelineExecError::InvalidSecretConfiguration => write!(f, "Invalid Secret Configuration: Duplicate secret names were found"),
            PipelineExecError::ReqwestError(reqe) => write!(f, "Pipeline Error: Reqwest Error: {reqe}"),
            PipelineExecError::IOError(ioe) => write!(f, "Pipeline Error: I/O Error Error: {ioe}"),
        }
    }
}

impl Error for PipelineExecError {}

impl From<kube::Error> for PipelineExecError {
    fn from(value: kube::Error) -> Self {
        PipelineExecError::KubernetesError(value)
    }
}

impl From<serde_json::Error> for PipelineExecError {
    fn from(value: serde_json::Error) -> Self {
        PipelineExecError::JsonEncodeError(value)
    }
}

impl From<kube::runtime::wait::Error> for PipelineExecError {
    fn from(value: kube::runtime::wait::Error) -> Self {
        PipelineExecError::KubeRuntimeWaitError(value)
    }
}

impl From<sqlx::Error> for PipelineExecError {
    fn from(value: sqlx::Error) -> Self {
        PipelineExecError::SqlError(value)
    }
}

impl From<reqwest::Error> for PipelineExecError {
    fn from(value: reqwest::Error) -> Self {
        PipelineExecError::ReqwestError(value)
    }
}

impl From<std::io::Error> for PipelineExecError {
    fn from(value: std::io::Error) -> Self {
        PipelineExecError::IOError(value)
    }
}