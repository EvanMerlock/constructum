use std::{path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::kube::VaultAnnotations;

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum PipelineStatus {
    InProgress,
    Complete,
    Failed
}

impl<'a> From<PipelineStatus> for &'a str {
    fn from(value: PipelineStatus) -> Self {
        match value {
            PipelineStatus::InProgress => "InProgress",
            PipelineStatus::Complete => "Complete",
            PipelineStatus::Failed => "Failed",
        }    
    }
}

impl<'a> From<&'a str> for PipelineStatus {
    fn from(value: &'a str) -> Self {
        match value {
            "InProgress" => PipelineStatus::InProgress,
            "Complete" => PipelineStatus::Complete,
            "Failed" => PipelineStatus::Failed,
            _ => panic!("invalid PipelineStatus")
        }
    }
}

impl From<String> for PipelineStatus {
    fn from(value: String) -> Self {
        match value.as_str() {
            "InProgress" => PipelineStatus::InProgress,
            "Complete" => PipelineStatus::Complete,
            "Failed" => PipelineStatus::Failed,
            _ => panic!("invalid PipelineStatus")
        }
    }
}


#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct Pipeline {
    version: u64,

    pub steps: Vec<PipelineStep>,
    pub secrets: Option<Vec<PipelineSecretConfig>>,
}

impl Pipeline {
    pub fn normalize(&mut self) {
        for step in self.steps.iter_mut() {
            step.normalize_name();
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct PipelineStep {
    pub name: String,
    pub image: String,
    pub pull: PipelineImagePullPref,
    pub commands: Vec<String>,
    pub secrets: Option<Vec<StepSecretConfig>>,
}

impl PipelineStep {
    fn normalize_name(&mut self) {
        let norm_one = self.name.trim().to_lowercase();
        let mut normalized = String::new();

        let spws: Vec<&str> = norm_one.split_whitespace().collect();
        
        for (idx, elem) in spws.iter().enumerate() {
            normalized.push_str(elem);

            if idx > 0 && idx < spws.len() - 1 {
                normalized.push('_');
            }
        }

        self.name = normalized;
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct StepSecretConfig {
    pub name: String,
    pub var_name: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum PipelineImagePullPref {
    Always,
}

pub struct PipelineJobConfig {
    pub pipeline: String,
    pub step: String,
    pub container: String,
    pub commands: Vec<String>,
    pub pipeline_working_directory: PathBuf,
    pub annotations: Option<VaultAnnotations>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct PipelineSecretConfig {
    pub name: String,
    pub location: String,
    pub key: String,
}