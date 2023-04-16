use serde::{Deserialize, Serialize};

use super::{PipelineStatus, PipelineStep};

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct JobContents {
    pub status: PipelineStatus,
    pub pipeline: CompletedPipeline,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct CompletedPipeline {
    pub steps: Vec<CompletedPipelineStep>,
}

impl CompletedPipeline {
    pub fn new() -> CompletedPipeline {
        CompletedPipeline { steps: Vec::new() }
    }
}

impl Default for CompletedPipeline {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct CompletedPipelineStep {
    pub name: String,
    pub image: String,
    pub commands: Vec<String>,
    pub status: StepStatus,
    pub log_key: Option<Vec<String>>,
}

impl CompletedPipelineStep {
    pub fn from_pipeline_step(pl_step: &PipelineStep, pl_status: StepStatus, log_names: Vec<String>) -> CompletedPipelineStep {
        CompletedPipelineStep { name: pl_step.name.clone(), image: pl_step.image.clone(), commands: pl_step.commands.clone(), status: pl_status, log_key: Some(log_names) }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum StepStatus {
    Success,
    Fail
}