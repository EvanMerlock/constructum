use std::path::Path;

use k8s_openapi::api::{core::v1::PersistentVolumeClaim, batch::v1::Job};
use kube::{Api, api::PostParams, runtime::wait::{await_condition, conditions}};
use s3::Bucket;
use tokio::{task, io::AsyncReadExt};
use uuid::Uuid;

use crate::{ConstructumState, pipeline::{Pipeline}, server::error::ConstructumServerError, git, kube::{build_client_pvc, put_pod_logs_to_s3, delete_job, delete_pvc}};

use super::{db_list_unfinished_jobs};

pub struct CreateJobPayload {
    html_url: String,
    name: String,
    commit_hash: String
}

impl CreateJobPayload {
    pub fn new(html_url: String, name: String, commit_hash: String) -> CreateJobPayload {
        CreateJobPayload { html_url, name, commit_hash }
    }
}

pub async fn create_job(payload: CreateJobPayload, state: ConstructumState) -> Result<Uuid, ConstructumServerError> {
    let pipeline_uuid = record_new_job_to_sql(payload, state.clone()).await?;
    assign_job_to_k8s(pipeline_uuid, state).await?;

    Ok(pipeline_uuid)
}

async fn record_new_job_to_sql(payload: CreateJobPayload, state: ConstructumState) -> Result<Uuid, ConstructumServerError> {
    let mut pipeline_file = git::get_pipeline_file(
        Path::new("E:\\Code\\.constructum\\build_cache"),
        payload.html_url.clone(),
        payload.name.clone(),
        payload.commit_hash.clone(),
    )
    .await?;
    let mut pipeline_contents = String::new();
    pipeline_file.read_to_string(&mut pipeline_contents).await?;

    let pipeline: Pipeline = serde_yaml::from_str(&pipeline_contents)?;
    println!("{pipeline:?}");

    let pipeline_uuid = Uuid::new_v4();
    
    {
        // record pipeline in SQL.
        // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
        let mut sql_connection = state.postgres.acquire().await?;
        sqlx::query("INSERT INTO constructum.jobs (id, repo_url, repo_name, commit_id, is_finished, job_json) VALUES ($1, $2, $3, $4, FALSE, NULL)")
            .bind(pipeline_uuid)
            .bind(&payload.html_url)
            .bind(&payload.name)
            .bind(&payload.commit_hash)
            .execute(&mut sql_connection).await?;
    }

    Ok(pipeline_uuid)
}

async fn assign_job_to_k8s(pipeline_uuid: Uuid, state: ConstructumState) -> Result<(), ConstructumServerError> {
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
    let data = crate::kube::build_client_job(pipeline_uuid, pipeline_client_name.clone(), state.container_name.clone())?;
    let _ = jobs.create(&PostParams::default(), &data).await?;

    // split this out to not block the response to the Git server
    task::spawn(async move {
        server_job(pipeline_client_name, pipeline_uuid, state.s3_bucket).await;
    });

    Ok(())
}

async fn server_job(pipeline_client_name: String, pipeline_uuid: Uuid, s3_bucket: Bucket) {
    let k8s_client = kube::Client::try_default().await.expect("failed to acquire k8s client");
    let jobs: Api<Job> = Api::namespaced(k8s_client, "constructum");
    let _ = await_condition(jobs.clone(), &pipeline_client_name, conditions::Condition::or(conditions::is_job_completed(), crate::kube::utils::is_job_failed())).await.expect("failed to wait on task");

    // record results
    match put_pod_logs_to_s3(pipeline_client_name.clone(), pipeline_client_name.to_string(), s3_bucket).await {
        Ok(()) => {},
        Err(e) => {
            println!("{e}");
        }
    };

    // clean up client job
    delete_job(&pipeline_client_name).await.expect("failed to delete job");
    delete_pvc(&pipeline_uuid.to_string()).await.expect("failed to delete job");
}

pub async fn restart_unfinished_jobs(state: ConstructumState) -> Result<(), ConstructumServerError> {
    let mut unfinished_jobs = db_list_unfinished_jobs(state.postgres.clone()).await?;

    // restart first N jobs, where N is defined by TODO: config
    unfinished_jobs.truncate(1);
    for unfinished in unfinished_jobs {
        assign_job_to_k8s(unfinished.job_uuid, state.clone()).await?;
    }
    Ok(())
}