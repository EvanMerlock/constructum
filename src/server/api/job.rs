pub mod db {
    use sqlx::PgPool;
    use uuid::Uuid;
    
    use crate::pipeline::JobInfo;
    
    pub async fn list_jobs(
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
    
    pub async fn get_job(
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
}

pub mod endpoints {
    use axum::{extract::State, Json};
    use uuid::Uuid;

    use crate::{ConstructumState, pipeline::{JobInfo}, server::error::ConstructumServerError};
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
        axum::extract::Path(job_id): axum::extract::Path<Uuid> ,
    ) -> Result<Json<JobInfo>, ConstructumServerError> {
    
        let pipeline_info = super::db::get_job(job_id, state.postgres).await?;
        
        Ok(Json(pipeline_info))
    }
}