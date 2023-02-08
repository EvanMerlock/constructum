use serde::{Deserialize, Serialize};


#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Pipeline {
    version: u64,

    steps: Vec<PipelineStep>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct PipelineStep {
    name: String,
    image: String,
    pull: PipelineImagePullPref,
    commands: Vec<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum PipelineImagePullPref {
    Always,
}