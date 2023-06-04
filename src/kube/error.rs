use std::{fmt::Display, error::Error};


#[derive(Debug)]
pub enum ConstructumKubeError {
    Kube(kube::Error),
    KubeRuntimeWait(kube::runtime::wait::Error),
}

impl Display for ConstructumKubeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstructumKubeError::Kube(kub) => write!(f, "Kubernetes Error: {kub}"),
            ConstructumKubeError::KubeRuntimeWait(wait) => write!(f, "Kubernetes Wait Error: {wait}"),
        }
    }
}

impl Error for ConstructumKubeError {}

impl From<kube::Error> for ConstructumKubeError {
    fn from(value: kube::Error) -> Self {
        Self::Kube(value)
    }
}

impl From<kube::runtime::wait::Error> for ConstructumKubeError {
    fn from(value: kube::runtime::wait::Error) -> Self {
        Self::KubeRuntimeWait(value)
    }
}