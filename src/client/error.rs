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
        todo!()
    }
}

impl Error for ConstructumClientError {}

impl From<ConstructumConfigError> for ConstructumClientError {
    fn from(value: ConstructumConfigError) -> Self {
        todo!()
    }
}

impl From<sqlx::Error> for ConstructumClientError {
    fn from(value: sqlx::Error) -> Self {
        todo!()
    }
}

impl From<tokio::io::Error> for ConstructumClientError {
    fn from(value: tokio::io::Error) -> Self {
        todo!()
    }
}

impl From<serde_yaml::Error> for ConstructumClientError {
    fn from(value: serde_yaml::Error) -> Self {
        todo!()
    }
}

impl From<PipelineExecError> for ConstructumClientError {
    fn from(value: PipelineExecError) -> Self {
        todo!()
    }
}

impl From<GitError> for ConstructumClientError {
    fn from(value: GitError) -> Self {
        todo!()
    }
}

#[derive(Debug)]
pub enum PipelineExecError {
    KubernetesError(kube::Error),
    JsonEncodeError(serde_json::Error),
    KubeRuntimeWaitError(kube::runtime::wait::Error),
}

impl Display for PipelineExecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for PipelineExecError {}

impl From<kube::Error> for PipelineExecError {
    fn from(value: kube::Error) -> Self {
        todo!()
    }
}

impl From<serde_json::Error> for PipelineExecError {
    fn from(value: serde_json::Error) -> Self {
        todo!()
    }
}

impl From<kube::runtime::wait::Error> for PipelineExecError {
    fn from(value: kube::runtime::wait::Error) -> Self {
        todo!()
    }
}