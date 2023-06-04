use std::{fmt::Display, error::Error};

use crate::kube::error::ConstructumKubeError;


#[derive(Debug)]
pub enum ConstructumRedisError {
    RedisClientError(redis::RedisError),
    KubernetesError(ConstructumKubeError),
}

impl Display for ConstructumRedisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstructumRedisError::RedisClientError(red) => write!(f, "Redis Error: {red}"),
            ConstructumRedisError::KubernetesError(kub) => write!(f, "Redis Kubernetes Error: {kub}"),
        }
    }
}

impl Error for ConstructumRedisError {}

impl From<ConstructumKubeError> for ConstructumRedisError {
    fn from(value: ConstructumKubeError) -> Self {
        Self::KubernetesError(value)
    }
}

impl From<redis::RedisError> for ConstructumRedisError {
    fn from(value: redis::RedisError) -> Self {
        Self::RedisClientError(value)
    }
}