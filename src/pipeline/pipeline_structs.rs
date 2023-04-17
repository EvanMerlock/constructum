use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum PipelineStatus {
    InProgress,
    Complete,
    Failed
}

impl<'a> Into<&'a str> for &'a PipelineStatus {
    fn into(self) -> &'a str {
        match self {
            PipelineStatus::InProgress => "InProgress",
            PipelineStatus::Complete => "Complete",
            PipelineStatus::Failed => "Failed",
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
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct PipelineStep {
    pub name: String,
    pub image: String,
    pub pull: PipelineImagePullPref,
    pub commands: Vec<String>,
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
}