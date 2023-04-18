pub mod db {
    use sqlx::{PgPool};
    use uuid::Uuid;

    use crate::pipeline::{completed::{CompletedPipelineStep, StepStatus}, PipelineStep};
    
    pub async fn list_steps(
        pool: PgPool
    ) -> Result<Vec<CompletedPipelineStep>, sqlx::Error> {
        let steps: Vec<CompletedPipelineStep> = {
            // retrieve pipeline info from Postgres
            // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
            let mut sql_connection = pool.acquire().await?;
            sqlx::query_as("SELECT * FROM constructum.steps")
                .fetch_all(&mut sql_connection).await?
        };

        Ok(steps)
    }

    pub async fn list_steps_for_job(
        pool: PgPool,
        job_id: Uuid
    ) -> Result<Vec<CompletedPipelineStep>, sqlx::Error> {
        let mut steps: Vec<CompletedPipelineStep> = {
            // retrieve pipeline info from Postgres
            // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
            let mut sql_connection = pool.acquire().await?;
            sqlx::query_as("SELECT * FROM constructum.steps WHERE job = $1")
                .bind(job_id)
                .fetch_all(&mut sql_connection).await?
        };
        steps.sort_by(|left: &CompletedPipelineStep, right: &CompletedPipelineStep| left.step_number.partial_cmp(&right.step_number).expect("failed to sort steps"));

        Ok(steps)
    }

    pub async fn insert_step(
        pool: PgPool,
        job_id: Uuid,
        step_num: i32,
        step: &PipelineStep
    ) -> Result<Uuid, sqlx::Error> {
        let mut sql_connection = pool.acquire().await?;
        let step_id = Uuid::new_v4();
        sqlx::query("INSERT INTO constructum.steps (id, job, step_seq, name, image, commands, status, log_keys) VALUES ($1, $2, $3, $4, $5, $6, $7, array[]::TEXT[])")
            .bind(step_id)
            .bind(job_id)
            .bind(step_num)
            .bind(&step.name)
            .bind(&step.image)
            .bind(&step.commands)
            .bind(Into::<&str>::into(&StepStatus::NotStarted))
            .execute(&mut sql_connection).await?;
        Ok(step_id)
    }

    pub async fn update_step_status(
        pool: PgPool,
        id: Uuid,
        status: StepStatus,
    ) -> Result<(), sqlx::Error> {
        let mut sql_connection = pool.acquire().await?;
        sqlx::query("UPDATE constructum.steps SET status = $2 WHERE id = $1")
            .bind(id)
            .bind(Into::<&str>::into(&status))
            .execute(&mut sql_connection).await?;
        Ok(())
    }

    pub async fn update_step_logs(
        pool: PgPool,
        id: Uuid,
        log_files: Vec<String>
    ) -> Result<(), sqlx::Error> {
        let mut sql_connection = pool.acquire().await?;
        sqlx::query("UPDATE constructum.steps SET log_keys = array_cat(log_keys, $2) WHERE id = $1")
            .bind(id)
            .bind(log_files)
            .execute(&mut sql_connection).await?;
        Ok(())
    }
}