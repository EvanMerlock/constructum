use std::{path::{Path, PathBuf}, str::FromStr, collections::HashMap};


use k8s_openapi::api::batch::v1::Job;
use kube::{Api, api::PostParams, runtime::wait::conditions};

use serde::{Serialize, Deserialize};

use tokio::{io::AsyncReadExt};
use uuid::Uuid;

use crate::{pipeline::{Pipeline, PipelineStatus, PipelineJobConfig, MaterializedSecretConfig, PipelineStep, MaterializedSecret}, config::Config, git, kube::{put_pod_logs_to_s3, delete_job}, server::{api::{job::{db::{get_job, complete_job}, JobInfo}, self, repo::RepoInfo, step::model::StepStatus}, self}, utils, ConstructumClientState, redis::logs_to_redis};

mod error;

#[cfg(test)]
mod tests;

pub use self::error::*;

pub async fn create_client_job(config: Config) -> Result<(), ConstructumClientError> {
    let pipeline_uuid = Uuid::from_str(config.pipeline_uuid.as_ref().expect("failed to get pipeline ID")).expect("failed to coerce to UUID");
    let vault_url = config.vault_server.clone().expect("failed to acquire vault server URL for client");
    let k8s_token = super::kube::read_kubernetes_token(vault_url.clone(), PathBuf::from("/var/run/secrets/tokens/vault-token")).await?;
    
    let state = ConstructumClientState::new(config).await?;

    let pipeline_info: JobInfo = get_job(pipeline_uuid, state.postgres()).await?;
    let repo_info: RepoInfo = server::api::repo::db::get_repo(pipeline_info.repo_id, state.postgres()).await?;
    // begin by initializing the workspace for future jobs
    let pipeline_file = git::pull_repository(Path::new("/data/"), repo_info.repo_url, repo_info.repo_name, pipeline_info.commit_id).await?;
    let pipeline_working_directory = pipeline_file.1;
    let mut pipeline_file = pipeline_file.0;
    let mut pipeline_contents = String::new();
    pipeline_file.read_to_string(&mut pipeline_contents).await?;

    let mut pipeline: Pipeline = serde_yaml::from_str(&pipeline_contents)?;
    pipeline.normalize();
    println!("{pipeline:?}");

    let materialized_secrets = build_pipeline_secrets(pipeline.clone(), vault_url.clone(), k8s_token).await?;

    let pipeline_status = execute_pipeline(pipeline.clone(), pipeline_info.job_uuid, pipeline_working_directory, &state, materialized_secrets).await?;
    println!("{pipeline_status:?}");

    complete_job(state.postgres(), pipeline_status, pipeline_uuid).await?;

    Ok(())
}

pub async fn build_pipeline_secrets(pipeline: Pipeline, vault_url: String, token: String) -> Result<MaterializedSecretConfig, PipelineExecError> {
    let secrets_requested = pipeline.secrets;

    match secrets_requested {
        Some(secrets) => {
            for (idx, secret) in secrets.iter().enumerate() {
                let mut second = secrets.clone();
                second.remove(idx);
                for secret_two in second {
                    if secret.name == secret_two.name {
                        // FAIL! cannot have 2 secrets with the same name.
                        return Err(PipelineExecError::InvalidSecretConfiguration);
                    }
                }
            }

            // validating all secrets exist
            for secret in secrets.iter() {

                #[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
                struct VaultSecretMetadata {
                    subkeys: HashMap<String, Option<String>>,
                }

                #[derive(Debug, PartialEq, Deserialize, Serialize, Clone)]
                struct VaultSecretResp {
                    data: VaultSecretMetadata
                }


                let resp = {
                    utils::get_with_auth(format!("{vault_url}/v1/constructum/subkeys/{}", secret.location), "X-Vault-Token", token.clone()).await?
                };
            
                let md = resp.json::<VaultSecretResp>().await?;

                if !md.data.subkeys.contains_key(&secret.key) {
                    // FAIL! secret not found!
                    return Err(PipelineExecError::InvalidSecretConfiguration);
                }

            }

            let materialized_secrets: Vec<MaterializedSecret> = secrets.into_iter().map(|x| MaterializedSecret::new(x.name, x.location, x.key)).collect();

            // validating all secrets in all steps exist within our materialized secrets
            for ss in pipeline.steps.iter().flat_map(|x| x.secrets.clone().unwrap_or(Vec::new())) {
                if materialized_secrets.iter().filter(|x| x.object_name == ss.name).count() == 0 {
                    // FAIL! secret does not exist for pipeline step.
                    return Err(PipelineExecError::InvalidSecretConfiguration);
                }
            }

            Ok(MaterializedSecretConfig::new(materialized_secrets))
        },
        None => Ok(MaterializedSecretConfig::default()),
    }
}

pub async fn build_step_secrets(step: PipelineStep, pipeline_secret_config: MaterializedSecretConfig) -> Result<Option<crate::kube::VaultAnnotations>, PipelineExecError> {
    let secrets_requested = step.secrets;
    let global_indexed_secrets = pipeline_secret_config.indexed_secrets();

    match secrets_requested {
        Some(step_secrets) => {
            let mut mat_secs = Vec::new();
            for ss in step_secrets {
                match global_indexed_secrets.get(&ss.name) {
                    Some(mat_sec) => mat_secs.push(mat_sec.clone()),
                    // should not happen, secrets should have been validated earlier
                    None => return Err(PipelineExecError::InvalidSecretConfiguration)
                }
            }

            let job_mat_secrets = MaterializedSecretConfig::new(mat_secs);

            let spc = crate::kube::build_vault_annotations(job_mat_secrets);

            Ok(Some(spc))
        },
        None => Ok(None),
    }
}

pub async fn execute_pipeline(pipeline: Pipeline, pipeline_uuid: uuid::Uuid, pipeline_working_directory: PathBuf, state: &ConstructumClientState, secrets: MaterializedSecretConfig) -> Result<PipelineStatus, PipelineExecError> {        
        // read in stages
        let k8s_client = kube::Client::try_default().await?;
        // execute stages as jobs on k8s

        let mut steps = Vec::new();
        for (step_num, step) in pipeline.steps.clone().into_iter().enumerate() {
            let step_uuid = api::step::db::insert_step(state.postgres(), pipeline_uuid, i32::try_from(step_num).expect("failed to convert step num"), &step).await?;
            steps.push((step_uuid, step));
        }

    
        let jobs: Api<Job> = Api::namespaced(k8s_client.clone(), "constructum");
        for (step_id, step) in steps {
            let name = step.name.clone();
            api::step::db::update_step_status(state.postgres(), step_id, StepStatus::InProgress).await?;

            // grab all secrets necessary for this step

            let secrets_generated = build_step_secrets(step.clone(), secrets.clone()).await?;

            // build corrected argument string for container

            let mut corrected_args = Vec::new();
            corrected_args.push(String::from("-c"));
            let mut args_to_correct = Vec::new();
            if let Some(secrets) = &secrets_generated {
                args_to_correct.append(&mut secrets.to_source_commands());
            }
            args_to_correct.append(&mut step.commands.clone());
            let fixed_arg = correct_args(args_to_correct);
            corrected_args.push(fixed_arg.expect("failed to build corrected args"));
            
            // create step cfg

            let pipeline_step_config = PipelineJobConfig {
                pipeline: pipeline_uuid.to_string(),
                step: name.clone(),
                container: step.image.clone(),
                commands: corrected_args,
                pipeline_working_directory: pipeline_working_directory.clone(),
                annotations: match secrets_generated.is_some() {
                    true => Some(secrets_generated.unwrap()),
                    false => None,
                },
            };

            // run the job on k8s

            let data = crate::kube::build_pipeline_job(pipeline_step_config)?;
            jobs.create(&PostParams::default(), &data.0).await?;

            // begin streaming logs to redis
            // TODO: job name is wrong, needs to be pipeline-UUID-container name.
            let logs_stream_fut = logs_to_redis(state.redis(), data.1.clone(), data.2.clone(), name.clone());


            // wait until the CI/CD job is complete or failed
    
            let pipeline_job_name = format!("pipeline-{pipeline_uuid}-{name}");
            let job_done_fut = kube::runtime::wait::await_condition(jobs.clone(), &pipeline_job_name, conditions::Condition::or(conditions::is_job_completed(), crate::kube::utils::is_job_failed()));

            let logs_stream_handle = tokio::spawn(logs_stream_fut);

            let res = tokio::join!(
                logs_stream_handle,
                job_done_fut
            );

            res.0.expect("failed to join")?;
            res.1?;

            // upload pod logs to S3
            let log_names = put_pod_logs_to_s3(data.1.clone(), Some(data.2), data.1, state.s3_bucket()).await?;

            // check if job failed. if so, bail from pipeline with pipelinestatus::failed

            let job_with_status = jobs.get_status(&data.0.metadata.name.expect("failed to find job name")).await?;
            if let Some(_pcond) = job_with_status.status.expect("failed to get job status").conditions.expect("failed to get job conditions").iter().find(|c| c.type_ == "Failed" && c.status == "True") {
                // delete the job
                api::step::db::update_step_status(state.postgres(), step_id, StepStatus::Fail).await?;
                api::step::db::update_step_logs(state.postgres(), step_id, log_names.clone()).await?;
                delete_job(&pipeline_job_name).await?;
                
                return Ok(PipelineStatus::Failed);
            }

            // delete the job
            api::step::db::update_step_status(state.postgres(), step_id, StepStatus::Success).await?;
            api::step::db::update_step_logs(state.postgres(), step_id, log_names.clone()).await?;
            delete_job(&pipeline_job_name).await?;
        }

        Ok(PipelineStatus::Complete)
}

fn correct_args(args_to_correct: Vec<String>) -> Option<String> {
    let mut sb = String::new();

    for (idx, arg) in args_to_correct.iter().enumerate() {
        sb.push_str(arg);
        sb.push(';');
        if idx < args_to_correct.len() - 1 {
            sb.push(' ');
        }
    }

    Some(sb)

}