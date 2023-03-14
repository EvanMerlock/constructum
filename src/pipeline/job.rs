use serde::{Deserialize, Serialize};
use uuid::Uuid;

use sqlx::{postgres::PgRow, Row};

use super::{PipelineStatus, Pipeline};


#[derive(Debug, Serialize)]
pub struct JobInfo {
    pub job_uuid: Uuid,
    pub repo_url: String,
    pub repo_name: String,
    pub commit_id: String,
    pub is_finished: bool,
    pub job_json: Option<JobContents>,
}

impl<'r> sqlx::FromRow<'r, PgRow> for JobInfo {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let uuid: Uuid = row.try_get("id")?;
        let repo_url: String = row.try_get("repo_url")?;
        let repo_name: String = row.try_get("repo_name")?;
        let commit_id: String = row.try_get("commit_id")?;
        let is_finished: bool = row.try_get("is_finished")?;
        let job_json: Option<sqlx::types::Json<JobContents>> = row.try_get("job_json")?;

        Ok(
            JobInfo { 
                job_uuid: uuid, 
                repo_url,
                repo_name,
                commit_id, 
                is_finished,
                job_json: job_json.map(|x| x.0)
            }
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JobContents {
    pub status: PipelineStatus,
    pub pipeline: Pipeline,
}