use serde::{Serialize};
use uuid::Uuid;

use sqlx::{postgres::PgRow, Row};

use super::{completed::{CompletedPipelineStep}, PipelineStatus};


#[derive(Debug, Serialize)]
pub struct JobInfo {
    pub job_uuid: Uuid,
    pub repo_url: String,
    pub repo_name: String,
    pub commit_id: String,
    pub is_finished: bool,
    pub status: PipelineStatus,
    pub steps: Option<Vec<CompletedPipelineStep>>
}

impl<'r> sqlx::FromRow<'r, PgRow> for JobInfo {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let uuid: Uuid = row.try_get("id")?;
        let repo_url: String = row.try_get("repo_url")?;
        let repo_name: String = row.try_get("repo_name")?;
        let commit_id: String = row.try_get("commit_id")?;
        let is_finished: bool = row.try_get("is_finished")?;
        let pipeline_status: String = row.try_get("status")?;

        Ok(
            JobInfo { 
                job_uuid: uuid, 
                repo_url,
                repo_name,
                commit_id, 
                is_finished,
                status: PipelineStatus::from(pipeline_status),
                steps: None
            }
        )
    }
}