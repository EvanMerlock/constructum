use serde::{Serialize, Deserialize};
use uuid::Uuid;

use sqlx::{postgres::PgRow, Row, FromRow};
use crate::{pipeline::{PipelineStatus}, server::api::step::model::CompletedPipelineStep};


#[derive(Debug, Serialize)]
pub struct JobInfo {
    pub job_uuid: Uuid,
    pub job_number: i32,
    pub repo_id: Uuid,
    pub commit_id: String,
    pub is_finished: bool,
    pub status: PipelineStatus,
    pub steps: Option<Vec<CompletedPipelineStep>>
}

impl<'r> sqlx::FromRow<'r, PgRow> for JobInfo {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let uuid: Uuid = row.try_get("id")?;
        let job_number: i32 = row.try_get("seq")?;
        let repo_id: Uuid = row.try_get("repo_id")?;
        let commit_id: String = row.try_get("commit_id")?;
        let is_finished: bool = row.try_get("is_finished")?;
        let pipeline_status: String = row.try_get("status")?;

        Ok(
            JobInfo { 
                job_uuid: uuid,
                job_number,
                repo_id,
                commit_id, 
                is_finished,
                status: PipelineStatus::from(pipeline_status),
                steps: None
            }
        )
    }
}