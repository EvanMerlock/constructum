use std::path::Path;

use k8s_openapi::api::{batch::v1::Job};
use kube::{Api, api::{PostParams, DeleteParams}, runtime::wait::{Condition, conditions}};
use tokio::io::AsyncReadExt;
use uuid::Uuid;

use crate::{pipeline::{Pipeline, PipelineStatus, PipelineJobConfig}, config::{Config, self}, git, ConstructumState};

mod error;

pub use self::error::*;

pub async fn create_client_job(config: Config) -> Result<(), ConstructumClientError> {
    // somewhere in here call execute_pipeline
    let pipeline_uuid = config.pipeline_uuid.clone().expect("failed to get pipeline ID");
    let (pool, bucket) = config::build_postgres_and_s3(config).await?;

    let pipeline_info: (Uuid, String, String, String, bool, serde_json::Value) = {
        // retrieve pipeline info from Postgres
        // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
        let mut sql_connection = pool.acquire().await?;
        sqlx::query_as("SELECT * FROM constructum.jobs WHERE id = $1")
            .bind(pipeline_uuid)
            .fetch_one(&mut sql_connection).await?
    };

    // begin by initializing the workspace for future jobs
    let mut pipeline_file = git::pull_repository(Path::new("/data/"), pipeline_info.1, pipeline_info.2, pipeline_info.3).await?;
    let mut pipeline_contents = String::new();
    pipeline_file.read_to_string(&mut pipeline_contents).await?;

    let pipeline: Pipeline = serde_yaml::from_str(&pipeline_contents)?;
    println!("{pipeline:?}");

    execute_pipeline(pipeline, pipeline_info.0).await?;

    
    Ok(())
}

pub async fn execute_pipeline(pipeline: Pipeline, pipeline_uuid: uuid::Uuid) -> Result<PipelineStatus, PipelineExecError> {
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
            };

            // run the job on k8s

            let data = crate::kube::build_pipeline_job(pipeline_step_config)?;
            jobs.create(&PostParams::default(), &data).await?;

            // wait until the CI/CD job is complete or failed
    
            kube::runtime::wait::await_condition(jobs.clone(), &name, conditions::Condition::or(conditions::is_job_completed(), is_job_failed())).await?;

            // check if job failed. if so, bail from pipeline with pipelinestatus::failed

            let job_with_status = jobs.get_status(&data.metadata.name.expect("failed to find job name")).await?;

            // upload pod logs to S3

            // delete the job
    
            jobs.delete(&name, &DeleteParams::background()).await?;
        }

        Ok(PipelineStatus::Complete)
}

pub fn is_job_failed() -> impl Condition<Job> {
    |obj: Option<&Job>| {
        if let Some(job) = &obj {
            if let Some(s) = &job.status {
                if let Some(conds) = &s.conditions {
                    if let Some(pcond) = conds.iter().find(|c| c.type_ == "Failed") {
                        return pcond.status == "True";
                    }
                }
            }
        }
        false
    }
}