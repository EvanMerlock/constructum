use serde::{Deserialize, Serialize};
use sqlx::{FromRow, postgres::PgRow, Row};

use super::{PipelineStep};

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

impl<'r> FromRow<'r, PgRow> for CompletedPipelineStep {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        // let id: Uuid = row.try_get("id")?;
        // let job_uuid: Uuid = row.try_get("job")?;
        let name: String = row.try_get("name")?;    
        let image: String = row.try_get("image")?;
        let commands: Vec<String> = row.try_get("commands")?;
        let status = StepStatus::from_row(row)?;
        let log_keys: Option<Vec<String>> = row.try_get("log_keys")?;
        Ok(
            CompletedPipelineStep { name, image, commands, status, log_key: log_keys }
        )
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone, Copy)]
pub enum StepStatus {
    NotStarted,
    InProgress,
    Success,
    Fail
}

impl<'a> Into<&'a str> for &'a StepStatus {
    fn into(self) -> &'a str {
        match self {
            StepStatus::NotStarted => "NotStarted",
            StepStatus::InProgress => "InProgress",
            StepStatus::Success => "Success",
            StepStatus::Fail => "Fail",
        }
    }
}

impl From<String> for StepStatus {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "NotStarted" => StepStatus::NotStarted,
            "InProgress" => StepStatus::InProgress,
            "Success" => StepStatus::Success,
            "Fail" => StepStatus::Fail,
            _ => panic!("bad stepstatus")
        }
    }
}

impl<'r> FromRow<'r, PgRow> for StepStatus {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let status_string: String = row.try_get("status")?;
        Ok(StepStatus::from(status_string))
    }
}