use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use super::JobInfo;
use crate::{
    pipeline::PipelineStatus,
    server::{api::step, CreateJobPayload},
};

pub async fn list_jobs(pool: PgPool) -> Result<Vec<JobInfo>, sqlx::Error> {
    let mut pipeline_info: Vec<JobInfo> = {
        // retrieve pipeline info from Postgres
        // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
        let mut sql_connection = pool.acquire().await?;
        sqlx::query_as("SELECT * FROM constructum.jobs")
            .fetch_all(&mut sql_connection)
            .await?
    };

    // TODO: this is bad. JOIN
    for info in pipeline_info.iter_mut() {
        let pipeline_step_info = step::db::list_steps_for_job(pool.clone(), info.job_uuid).await?;
        info.steps = Some(pipeline_step_info);
    }

    Ok(pipeline_info)
}

#[tracing::instrument(skip(pool))]
pub async fn list_jobs_for_repo(repo_id: Uuid, pool: PgPool) -> Result<Vec<JobInfo>, sqlx::Error> {
    let mut pipeline_info: Vec<JobInfo> = {
        // retrieve pipeline info from Postgres
        // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
        let mut sql_connection = pool.acquire().await?;
        sqlx::query_as("SELECT * FROM constructum.jobs WHERE repo_id = $1")
            .bind(repo_id)
            .fetch_all(&mut sql_connection)
            .await?
    };

    // TODO: this is bad. JOIN
    for info in pipeline_info.iter_mut() {
        let pipeline_step_info = step::db::list_steps_for_job(pool.clone(), info.job_uuid).await?;
        info.steps = Some(pipeline_step_info);
    }

    Ok(pipeline_info)
}

pub async fn get_job(job_id: Uuid, pool: PgPool) -> Result<JobInfo, sqlx::Error> {
    let mut pipeline_info: JobInfo = {
        // retrieve pipeline info from Postgres
        // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
        let mut sql_connection = pool.acquire().await?;
        sqlx::query_as("SELECT * FROM constructum.jobs WHERE id = $1")
            .bind(job_id)
            .fetch_one(&mut sql_connection)
            .await?
    };

    let pipeline_step_info = step::db::list_steps_for_job(pool, pipeline_info.job_uuid).await?;
    pipeline_info.steps = Some(pipeline_step_info);

    Ok(pipeline_info)
}

pub async fn list_unfinished_jobs(pool: PgPool) -> Result<Vec<JobInfo>, sqlx::Error> {
    let mut pipeline_info: Vec<JobInfo> = {
        // retrieve pipeline info from Postgres
        // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
        let mut sql_connection = pool.acquire().await?;
        sqlx::query_as("SELECT * FROM constructum.jobs WHERE is_finished = FALSE")
            .fetch_all(&mut sql_connection)
            .await?
    };

    // TODO: this is bad. JOIN
    for info in pipeline_info.iter_mut() {
        let pipeline_step_info = step::db::list_steps_for_job(pool.clone(), info.job_uuid).await?;
        info.steps = Some(pipeline_step_info);
    }

    Ok(pipeline_info)
}

pub async fn create_job(
    pool: PgPool,
    pipeline_uuid: Uuid,
    build_number: i32,
    repo_uuid: Uuid,
    payload: CreateJobPayload,
) -> Result<(), sqlx::Error> {
    let mut sql_connection = pool.acquire().await?;
    sqlx::query("INSERT INTO constructum.jobs (id, seq, repo_id, commit_id, is_finished, status) VALUES ($1, $2, $3, $4, FALSE, 'InProgress')")
        .bind(pipeline_uuid)
        .bind(build_number)
        .bind(repo_uuid)
        .bind(&payload.commit_hash)
        .execute(&mut sql_connection).await?;
    Ok(())
}

pub async fn complete_job(
    pool: PgPool,
    status: PipelineStatus,
    job_id: Uuid,
) -> Result<(), sqlx::Error> {
    let mut sql_connection = pool.acquire().await?;
    sqlx::query("UPDATE constructum.jobs SET is_finished = TRUE, status = $1 WHERE id = $2")
        .bind(Into::<&str>::into(status))
        .bind(job_id)
        .execute(&mut sql_connection)
        .await?;
    Ok(())
}

pub async fn get_job_log_ids(pool: PgPool, job_id: Uuid) -> Result<Vec<String>, sqlx::Error> {
    #[derive(FromRow)]
    struct JobLogId {
        log_key: String,
    }

    let job_ids: Vec<String> = {
        // retrieve pipeline info from Postgres
        // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
        let mut sql_connection = pool.acquire().await?;
        // TODO: broken
        sqlx::query_as("SELECT UNNEST(logs_keys) AS log_key FROM constructum.steps WHERE job = $1")
            .bind(job_id)
            .fetch_all(&mut sql_connection).await?
    }.into_iter().map(|x: JobLogId| x.log_key.trim_matches('"').to_string()).collect();

    Ok(job_ids)
}
