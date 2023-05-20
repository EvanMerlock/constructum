use axum::{extract::State, Json};
use uuid::Uuid;

use crate::{ConstructumState, server::error::ConstructumServerError, s3buckets};
use super::JobInfo;

pub async fn list_jobs(
    State(state): State<ConstructumState>
) -> Result<Json<Vec<JobInfo>>, ConstructumServerError> {

    let pipeline_info = super::db::list_jobs(state.postgres).await?;

    Ok(Json(pipeline_info))
}

pub async fn get_job(
    State(state): State<ConstructumState>,
    axum::extract::Path(job_id): axum::extract::Path<Uuid>,
) -> Result<Json<JobInfo>, ConstructumServerError> {

    let pipeline_info = super::db::get_job(job_id, state.postgres).await?;
    
    Ok(Json(pipeline_info))
}

pub async fn get_job_logs(
    State(state): State<ConstructumState>,
    axum::extract::Path(job_id): axum::extract::Path<Uuid>,
) -> Result<Json<Vec<String>>, ConstructumServerError> {
    use ::futures::future::join_all;

    let files_to_pull = super::db::get_job_log_ids(state.postgres, job_id).await?;
    let result = join_all(files_to_pull.into_iter().map(|file_name| s3buckets::get_file_from_s3(file_name, state.s3_bucket.clone()))).await.into_iter().map(|x| x.expect("failed to grab s3 logs")).collect();
    Ok(Json(result))
}