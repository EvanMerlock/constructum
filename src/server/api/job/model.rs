use serde::{Serialize, Deserialize};
use uuid::Uuid;

use sqlx::{postgres::PgRow, Row, FromRow};
use crate::pipeline::{PipelineStatus};


#[derive(Debug, Serialize)]
pub struct JobInfo {
    pub job_uuid: Uuid,
    pub repo_id: Uuid,
    pub commit_id: String,
    pub is_finished: bool,
    pub status: PipelineStatus,
    pub steps: Option<Vec<CompletedPipelineStep>>
}

impl<'r> sqlx::FromRow<'r, PgRow> for JobInfo {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let uuid: Uuid = row.try_get("id")?;
        let repo_id: Uuid = row.try_get("repo_id")?;
        let commit_id: String = row.try_get("commit_id")?;
        let is_finished: bool = row.try_get("is_finished")?;
        let pipeline_status: String = row.try_get("status")?;

        Ok(
            JobInfo { 
                job_uuid: uuid, 
                repo_id,
                commit_id, 
                is_finished,
                status: PipelineStatus::from(pipeline_status),
                steps: None
            }
        )
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
pub struct CompletedPipelineStep {
    pub name: String,
    pub step_number: i32,
    pub image: String,
    pub commands: Vec<String>,
    pub status: StepStatus,
    pub log_key: Option<Vec<String>>,
}

impl<'r> FromRow<'r, PgRow> for CompletedPipelineStep {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        // let id: Uuid = row.try_get("id")?;
        // let job_uuid: Uuid = row.try_get("job")?;
        let name: String = row.try_get("name")?;    
        let image: String = row.try_get("image")?;
        let step_num: i32 = row.try_get("step_seq")?;
        let commands: Vec<String> = row.try_get("commands")?;
        let status = StepStatus::from_row(row)?;
        let log_keys: Option<Vec<String>> = row.try_get("log_keys")?;
        Ok(
            CompletedPipelineStep { name, step_number: step_num, image, commands, status, log_key: log_keys }
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