use sqlx::PgPool;
use uuid::Uuid;

use crate::pipeline::JobInfo;

pub async fn db_list_jobs(
    pool: PgPool,
) -> Result<Vec<JobInfo>, sqlx::Error> {

    let pipeline_info: Vec<JobInfo> = {
        // retrieve pipeline info from Postgres
        // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
        let mut sql_connection = pool.acquire().await?;
        sqlx::query_as("SELECT * FROM constructum.jobs")
            .fetch_all(&mut sql_connection).await?
    };
    
    Ok(pipeline_info)
}

pub async fn db_get_job(
    job_id: Uuid,
    pool: PgPool,
) -> Result<JobInfo, sqlx::Error> {

    let pipeline_info: JobInfo = {
        // retrieve pipeline info from Postgres
        // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
        let mut sql_connection = pool.acquire().await?;
        sqlx::query_as("SELECT * FROM constructum.jobs WHERE id = $1")
            .bind(job_id)
            .fetch_one(&mut sql_connection).await?
    };
    
    Ok(pipeline_info)
}

pub async fn db_list_unfinished_jobs(
    pool: PgPool
) -> Result<Vec<JobInfo>, sqlx::Error> {

    let pipeline_info: Vec<JobInfo> = {
        // retrieve pipeline info from Postgres
        // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
        let mut sql_connection = pool.acquire().await?;
        sqlx::query_as("SELECT * FROM constructum.jobs WHERE is_finished = FALSE")
            .fetch_all(&mut sql_connection).await?
    };
    
    Ok(pipeline_info)
}