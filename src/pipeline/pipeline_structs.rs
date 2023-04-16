use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum PipelineStatus {
    Complete,
    Failed
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