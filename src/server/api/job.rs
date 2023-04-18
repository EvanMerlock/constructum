pub mod db {
    use sqlx::{PgPool, FromRow};
    use uuid::Uuid;
    
    use crate::{pipeline::{JobInfo, PipelineStatus}, server::{CreateJobPayload, api::step}};
    
    pub async fn list_jobs(
        pool: PgPool,
    ) -> Result<Vec<JobInfo>, sqlx::Error> {    
        let mut pipeline_info: Vec<JobInfo> = {
            // retrieve pipeline info from Postgres
            // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
            let mut sql_connection = pool.acquire().await?;
            sqlx::query_as("SELECT * FROM constructum.jobs")
                .fetch_all(&mut sql_connection).await?
        };

        // TODO: this is bad. JOIN
        for info in pipeline_info.iter_mut() {
            let pipeline_step_info = step::db::list_steps_for_job(pool.clone(), info.job_uuid).await?;
            info.steps = Some(pipeline_step_info);
        }
        
        Ok(pipeline_info)
    }
    
    pub async fn get_job(
        job_id: Uuid,
        pool: PgPool,
    ) -> Result<JobInfo, sqlx::Error> {
    
        let mut pipeline_info: JobInfo = {
            // retrieve pipeline info from Postgres
            // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
            let mut sql_connection = pool.acquire().await?;
            sqlx::query_as("SELECT * FROM constructum.jobs WHERE id = $1")
                .bind(job_id)
                .fetch_one(&mut sql_connection).await?
        };

        let pipeline_step_info = step::db::list_steps_for_job(pool, pipeline_info.job_uuid).await?;
        pipeline_info.steps = Some(pipeline_step_info);
        
        Ok(pipeline_info)
    }
    
    pub async fn list_unfinished_jobs(
        pool: PgPool
    ) -> Result<Vec<JobInfo>, sqlx::Error> {
    
        let mut pipeline_info: Vec<JobInfo> = {
            // retrieve pipeline info from Postgres
            // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
            let mut sql_connection = pool.acquire().await?;
            sqlx::query_as("SELECT * FROM constructum.jobs WHERE is_finished = FALSE")
                .fetch_all(&mut sql_connection).await?
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
        payload: CreateJobPayload
    ) -> Result<(), sqlx::Error> {
        let mut sql_connection = pool.acquire().await?;
        sqlx::query("INSERT INTO constructum.jobs (id, repo_url, repo_name, commit_id, is_finished, status) VALUES ($1, $2, $3, $4, FALSE, 'InProgress')")
            .bind(pipeline_uuid)
            .bind(&payload.html_url)
            .bind(&payload.name)
            .bind(&payload.commit_hash)
            .execute(&mut sql_connection).await?;
        Ok(())
    }

    pub async fn complete_job(pool: PgPool, status: PipelineStatus, job_id: Uuid) -> Result<(), sqlx::Error> {
        let mut sql_connection = pool.acquire().await?;
        sqlx::query("UPDATE constructum.jobs SET is_finished = TRUE, status = $1 WHERE id = $2")
            .bind(Into::<&str>::into(&status))
            .bind(job_id).execute(&mut sql_connection).await?;
        Ok(())
    }

    pub async fn get_job_log_ids(pool: PgPool, job_id: Uuid) -> Result<Vec<String>, sqlx::Error> {
        #[derive(FromRow)]
        struct JobLogId {
            log_key: String
        }

        let job_ids: Vec<String> = {
            // retrieve pipeline info from Postgres
            // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
            let mut sql_connection = pool.acquire().await?;
            sqlx::query_as("SELECT json_array_elements(json_array_elements(job_json::json->'pipeline'->'steps')->'log_key')::text AS log_key FROM constructum.jobs WHERE id = $1")
                .bind(job_id)
                .fetch_all(&mut sql_connection).await?
        }.into_iter().map(|x: JobLogId| x.log_key.trim_matches('"').to_string()).collect();

        Ok(job_ids)
    }
}

pub mod endpoints {
    use axum::{extract::State, Json};
    use uuid::Uuid;

    use crate::{ConstructumState, pipeline::{JobInfo}, server::error::ConstructumServerError, s3buckets};
    #[axum_macros::debug_handler]
    pub async fn list_jobs(
        State(state): State<ConstructumState>
    ) -> Result<Json<Vec<JobInfo>>, ConstructumServerError> {
    
        let pipeline_info = super::db::list_jobs(state.postgres).await?;
    
        Ok(Json(pipeline_info))
    }
    
    #[axum_macros::debug_handler]
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
}