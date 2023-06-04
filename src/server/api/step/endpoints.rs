use axum::{extract::State, Json, http::StatusCode, response::IntoResponse};
use futures::future::join_all;
use uuid::Uuid;

use crate::{server::error::ConstructumServerError, ConstructumServerState};

use super::model::{StepLogs, CompletedPipelineStep};

pub async fn get_log_for_step(
    State(state): State<ConstructumServerState>,
    axum::extract::Path((job_id, step_id)): axum::extract::Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, ConstructumServerError> {
    
    let step: CompletedPipelineStep = super::db::get_step(state.postgres(), step_id).await?;
    let job = super::super::job::db::get_job(job_id, state.postgres()).await?;

    let log = crate::redis::grab_log_from_redis(state.redis(), format!("pipeline-{}-{}", job.job_uuid, step.name.clone()), step.name).await?;

    Ok(match log {
        Some(log_str) => (StatusCode::OK, Json(StepLogs::Logs(log_str))),
        None => {
            // TODO: this should only return when it matches job and step
            let log_keys = super::db::get_logs_for_step(state.postgres(), step_id).await?;
            let result: Vec<String> = join_all(log_keys.into_iter().map(|file_name| crate::s3buckets::get_file_from_s3(file_name, state.s3_bucket()))).await.into_iter().map(|x| x.expect("failed to grab s3 logs")).collect();

            if result.is_empty() {
                (StatusCode::NOT_FOUND, Json(StepLogs::None))
            } else {
                (StatusCode::OK, Json(StepLogs::ManyLogs(result)))
            }
        },
    })

}