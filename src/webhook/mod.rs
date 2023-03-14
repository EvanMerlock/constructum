use std::path::Path;

use axum::{Json, extract::State};
use k8s_openapi::api::{batch::v1::Job, core::v1::{PersistentVolumeClaim, Pod}};
use kube::{
    api::{PostParams, LogParams, DeleteParams},
    Api, runtime::wait::{await_condition, conditions},
};
use tokio::{io::AsyncReadExt, task};
use uuid::Uuid;

use crate::{git, pipeline::Pipeline, ConstructumState, kube::{build_client_pvc, put_pod_logs_to_s3}};

use self::{error::ConstructumWebhookError, payload::GitWebhookPayload};

pub mod error;
pub mod payload;

#[axum_macros::debug_handler]
pub async fn webhook(
    State(state): State<ConstructumState>,
    Json(payload): Json<GitWebhookPayload>,
) -> axum::response::Result<(), ConstructumWebhookError> {
    let mut pipeline_file = git::get_pipeline_file(
        Path::new("E:\\Code\\.constructum\\build_cache"),
        payload.repository.html_url.clone(),
        payload.repository.name.clone(),
        payload.after.clone(),
    )
    .await?;
    let mut pipeline_contents = String::new();
    pipeline_file.read_to_string(&mut pipeline_contents).await?;

    let pipeline: Pipeline = serde_yaml::from_str(&pipeline_contents)?;
    println!("{pipeline:?}");

    let pipeline_uuid = Uuid::new_v4();
    let pipeline_name = format!("pipeline-{pipeline_uuid}");
    let pipeline_client_name = format!("{pipeline_name}-client");
    
    {
        // record pipeline in SQL.
        // we should release the connection ASAP so that we do not work steal while doing more computationally intensive work.
        let mut sql_connection = state.postgres.acquire().await?;
        sqlx::query("INSERT INTO constructum.jobs (id, repo_url, repo_name, commit_id, is_finished, job_json) VALUES ($1, $2, $3, $4, FALSE, NULL)")
            .bind(pipeline_uuid)
            .bind(&payload.repository.html_url)
            .bind(&payload.repository.name)
            .bind(&payload.after)
            .execute(&mut sql_connection).await?;
    }

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
        let k8s_client = kube::Client::try_default().await.expect("failed to acquire k8s client");
        let jobs: Api<Job> = Api::namespaced(k8s_client, "constructum");
        let _ = await_condition(jobs.clone(), &pipeline_client_name, conditions::Condition::or(conditions::is_job_completed(), crate::kube::utils::is_job_failed())).await.expect("failed to wait on task");

        // record results
        put_pod_logs_to_s3(pipeline_client_name.clone(), pipeline_client_name.to_string(), state.s3_bucket).await.expect("failed to put pod logs to s3");

        // clean up client job
        jobs.delete(&pipeline_client_name, &DeleteParams::default()).await.expect("failed to delete job");
    });

    Ok(())
}
