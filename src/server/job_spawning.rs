use std::path::Path;

use k8s_openapi::api::{core::v1::PersistentVolumeClaim, batch::v1::Job};
use kube::{Api, api::PostParams, runtime::wait::{await_condition, conditions}};
use tokio::{task, io::AsyncReadExt};
use tracing::error;
use uuid::Uuid;

use crate::{ConstructumServerState, pipeline::Pipeline, server::error::ConstructumServerError, git, kube::{build_client_pvc, put_pod_logs_to_s3, delete_job, delete_pvc}, redis::logs_to_redis};

use super::api::job::db::list_unfinished_jobs;

pub struct CreateJobPayload {
    pub repo_id: i32,
    pub html_url: String,
    pub name: String,
    pub commit_hash: String
}

impl CreateJobPayload {
    pub fn new(repo_id: i32, html_url: String, name: String, commit_hash: String) -> CreateJobPayload {
        CreateJobPayload { repo_id, html_url, name, commit_hash }
    }
}

pub async fn create_job(payload: CreateJobPayload, state: ConstructumServerState) -> Result<Uuid, ConstructumServerError> {
    let pipeline_uuid = record_new_job_to_sql(payload, state.clone()).await?;
    assign_job_to_k8s(pipeline_uuid, state).await?;

    Ok(pipeline_uuid)
}

async fn record_new_job_to_sql(payload: CreateJobPayload, state: ConstructumServerState) -> Result<Uuid, ConstructumServerError> {
    // checking for existence
    let repo_ref = 
        super::api::repo::db::get_repo_by_git_id(payload.repo_id, state.postgres())
            .await?
            .ok_or(ConstructumServerError::NoRepoFound)?;

    let mut pipeline_file = git::get_pipeline_file(
        Path::new(&state.build_cache_location()),
        payload.html_url.clone(),
        payload.name.clone(),
        payload.commit_hash.clone(),
    )
    .await?;
    let mut pipeline_contents = String::new();
    pipeline_file.read_to_string(&mut pipeline_contents).await?;

    let mut pipeline: Pipeline = serde_yaml::from_str(&pipeline_contents)?;
    pipeline.normalize();
    println!("{pipeline:?}");

    let pipeline_uuid = Uuid::new_v4();
    
    super::api::job::db::create_job(state.postgres(), pipeline_uuid, repo_ref.builds_executed+1, repo_ref.repo_uuid, payload).await?;
    super::api::repo::db::update_repo_seq(state.postgres(), repo_ref.repo_uuid, repo_ref.builds_executed+1).await?;

    Ok(pipeline_uuid)
}

async fn assign_job_to_k8s(pipeline_uuid: Uuid, state: ConstructumServerState) -> Result<(), ConstructumServerError> {
    let pipeline_name = format!("pipeline-{pipeline_uuid}");
    let pipeline_client_name = format!("{pipeline_name}-client");

    // create PVC on server process
    let k8s_client = kube::Client::try_default().await?;
    let pvcs: Api<PersistentVolumeClaim> = Api::namespaced(k8s_client, "constructum");
    let pvc_data = build_client_pvc(pipeline_uuid)?;
    pvcs.create(&PostParams::default(), &pvc_data).await?;
    // create client job
    let k8s_client = kube::Client::try_default().await?;
    let jobs: Api<Job> = Api::namespaced(k8s_client, "constructum");
    let data = crate::kube::build_client_job(pipeline_uuid, pipeline_client_name.clone(), state.container_name(), Some(String::from("constructum-client-validate")))?;
    let _ = jobs.create(&PostParams::default(), &data).await?;

    {
        // only handle the lock here
        state.current_jobs().write().expect("lock poisoned").insert(pipeline_uuid);
    }

    // split this out to not block the response to the Git server
    task::spawn(async move {
        server_job(pipeline_client_name, pipeline_uuid, state).await;
    });

    Ok(())
}

async fn server_job(pipeline_client_name: String, pipeline_uuid: Uuid, state: ConstructumServerState) {
    let k8s_client = kube::Client::try_default().await.expect("failed to acquire k8s client");
    let jobs: Api<Job> = Api::namespaced(k8s_client, "constructum");
    let logs_stream_fut = logs_to_redis(state.redis(), pipeline_client_name.clone(), format!("{pipeline_client_name}-container"), String::from("_client"));

    let job_done_fut = await_condition(jobs.clone(), &pipeline_client_name, conditions::Condition::or(conditions::is_job_completed(), crate::kube::utils::is_job_failed()));
    let log_stream_handle = tokio::spawn(logs_stream_fut);

    let res = tokio::join!(
        log_stream_handle,
        job_done_fut
    );

    match res.0.expect("failed to join") {
        Ok(_) => {},
        Err(err) => error!("Failed: {err}"),
    }

    match res.1 {
        Ok(_) => {},
        Err(err) => error!("Failed: {err}"),
    };

    // TODO: check for job cancellation and set job status correctly

    // record results
    match put_pod_logs_to_s3(pipeline_client_name.clone(), None, pipeline_client_name.to_string(), state.s3_bucket()).await {
        Ok(_) => {},
        Err(e) => {
            println!("{e}");
        }
    };

    // clean up client job
    delete_job(&pipeline_client_name).await.expect("failed to delete job");
    delete_pvc(&pipeline_uuid.to_string()).await.expect("failed to delete job");

    state.current_jobs().write().expect("lock poisoned").remove(&pipeline_uuid);
}

pub async fn restart_unfinished_jobs(state: ConstructumServerState) -> Result<(), ConstructumServerError> {
    let unfinished_jobs = list_unfinished_jobs(state.postgres()).await?;

    // restart first N jobs, where N is defined by TODO: config
    for unfinished in unfinished_jobs {
        if !state.current_jobs().read().expect("lock poisoned").contains(&unfinished.job_uuid) {
            assign_job_to_k8s(unfinished.job_uuid, state.clone()).await?;
            break;
        }
    }
    Ok(())
}