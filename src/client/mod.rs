use std::{path::{Path, PathBuf}, str::FromStr};

use k8s_openapi::api::{batch::v1::Job};
use kube::{Api, api::{PostParams, DeleteParams}, runtime::wait::{Condition, conditions}};
use s3::Bucket;
use tokio::io::AsyncReadExt;
use uuid::Uuid;

use crate::{pipeline::{Pipeline, PipelineStatus, PipelineJobConfig, JobInfo, JobContents}, config::{Config, self}, git, ConstructumState, kube::{put_pod_logs_to_s3, delete_job}};

mod error;

pub use self::error::*;

pub async fn create_client_job(config: Config) -> Result<(), ConstructumClientError> {
    // somewhere in here call execute_pipeline
    let pipeline_uuid = Uuid::from_str(config.pipeline_uuid.as_ref().expect("failed to get pipeline ID")).expect("failed to coerce to UUID");
    
    let (pool, bucket) = config::build_postgres_and_s3(config).await?;

    let pipeline_info: JobInfo = {
        // retrieve pipeline info from Postgres
        // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
        let mut sql_connection = pool.acquire().await?;
        sqlx::query_as("SELECT * FROM constructum.jobs WHERE id = $1")
            .bind(pipeline_uuid)
            .fetch_one(&mut sql_connection).await?
    };

    // begin by initializing the workspace for future jobs
    let pipeline_file = git::pull_repository(Path::new("/data/"), pipeline_info.repo_url, pipeline_info.repo_name, pipeline_info.commit_id).await?;
    let pipeline_working_directory = pipeline_file.1;
    let mut pipeline_file = pipeline_file.0;
    let mut pipeline_contents = String::new();
    pipeline_file.read_to_string(&mut pipeline_contents).await?;

    let pipeline: Pipeline = serde_yaml::from_str(&pipeline_contents)?;
    println!("{pipeline:?}");

    let pipeline_status = execute_pipeline(pipeline.clone(), pipeline_info.job_uuid, pipeline_working_directory, bucket).await?;
    println!("{pipeline_status:?}");

    {
        let job_info = JobContents {
            status: pipeline_status,
            pipeline,
        };
        let mut sql_connection = pool.acquire().await?;
        sqlx::query("UPDATE constructum.jobs SET is_finished = TRUE, job_json = $1 WHERE id = $2")
            .bind(serde_json::to_value(&job_info).expect("failed to coerce pipeline"))
            .bind(pipeline_info.job_uuid).execute(&mut sql_connection).await?;
    }

    Ok(())
}

pub async fn execute_pipeline(pipeline: Pipeline, pipeline_uuid: uuid::Uuid, pipeline_working_directory: PathBuf, bucket: Bucket) -> Result<PipelineStatus, PipelineExecError> {
        // read in stages
        let k8s_client = kube::Client::try_default().await?;
        // execute stages as jobs on k8s
    
        let jobs: Api<Job> = Api::namespaced(k8s_client, "constructum");
        for step in pipeline.steps {
            let name = step.name;

            // build corrected argument string for container

            let mut corrected_args = Vec::new();
            corrected_args.push(String::from("-c"));
            let fixed_arg = step.commands.into_iter().reduce(|mut acc, next| {
                acc.push_str(&next);
                acc.push(';');
                acc
            });
            corrected_args.push(fixed_arg.expect("failed to build corrected args"));

            // create step cfg

            let pipeline_step_config = PipelineJobConfig {
                pipeline: pipeline_uuid.to_string(),
                step: name.clone(),
                container: step.image,
                commands: corrected_args,
                pipeline_working_directory: pipeline_working_directory.clone(),
            };

            // run the job on k8s

            let data = crate::kube::build_pipeline_job(pipeline_step_config)?;
            jobs.create(&PostParams::default(), &data.0).await?;

            // wait until the CI/CD job is complete or failed
    
            let pipeline_job_name = format!("pipeline-{pipeline_uuid}-{name}");
            kube::runtime::wait::await_condition(jobs.clone(), &pipeline_job_name, conditions::Condition::or(conditions::is_job_completed(), crate::kube::utils::is_job_failed())).await?;

            // upload pod logs to S3

            put_pod_logs_to_s3(data.1.clone(), data.1, bucket.clone()).await?;

            // check if job failed. if so, bail from pipeline with pipelinestatus::failed

            let job_with_status = jobs.get_status(&data.0.metadata.name.expect("failed to find job name")).await?;
            if let Some(_pcond) = job_with_status.status.expect("failed to get job status").conditions.expect("failed to get job conditions").iter().find(|c| c.type_ == "Failed" && c.status == "True") {
                // delete the job
                delete_job(&pipeline_job_name).await?;
                
                return Ok(PipelineStatus::Failed);
            }

            // delete the job
            delete_job(&pipeline_job_name).await?;
        }

        Ok(PipelineStatus::Complete)
}