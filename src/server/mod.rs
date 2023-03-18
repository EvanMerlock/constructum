use axum::{extract::State, Json};
use uuid::Uuid;

use crate::{ConstructumState, pipeline::{JobInfo}, server::error::ConstructumServerError};

pub(crate) mod error;
mod job_spawning;
mod job_db;

pub use self::job_spawning::*;
pub use self::job_db::*;

#[axum_macros::debug_handler]
pub async fn list_jobs(
    State(state): State<ConstructumState>
) -> Result<Json<Vec<JobInfo>>, ConstructumServerError> {

    let pipeline_info = job_db::db_list_jobs(state.postgres).await?;

    Ok(Json(pipeline_info))
}

#[axum_macros::debug_handler]
pub async fn get_job(
    State(state): State<ConstructumState>,
    axum::extract::Path(job_id): axum::extract::Path<Uuid> ,
) -> Result<Json<JobInfo>, ConstructumServerError> {

    let pipeline_info = job_db::db_get_job(job_id, state.postgres).await?;
    
    Ok(Json(pipeline_info))
}