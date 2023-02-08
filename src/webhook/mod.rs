use std::{path::Path, env};

use axum::{Json};
use git2::{RemoteCallbacks, Cred, FetchOptions, build::RepoBuilder};
use tokio::{fs::File, io::AsyncReadExt};

use crate::pipeline::Pipeline;

use self::{payload::GitWebhookPayload, error::ConstructumWebhookError};

pub mod payload;
pub mod error;


pub async fn webhook(Json(payload): Json<GitWebhookPayload>) -> axum::response::Result<(), ConstructumWebhookError> {
    // clone repo
    let _repository = tokio::task::spawn_blocking(move || {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                // TODO: Should be read from config
                Path::new(&format!("{}/.ssh/id_ed25519", env::var("HOME").unwrap())),
                None,
            )
        });
    
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
    
        let mut repo_builder = RepoBuilder::new();
        repo_builder.fetch_options(fetch_options);
        repo_builder.clone(&payload.repository.ssh_url, Path::new("E:\\Code\\.constructum\\build_cache"))
    }).await.unwrap()?;
    // check for .constructum.yml

    let pipeline_file_string = format!("E:\\Code\\.constructum\\build_cache\\{}", payload.repository.name);
    let pipeline_file_path = Path::new(&pipeline_file_string);
    let mut pipeline_file = File::open(pipeline_file_path).await?;

    let mut pipeline_contents = String::new();
    pipeline_file.read_to_string(&mut pipeline_contents).await?;

    let pipeline: Pipeline = serde_yaml::from_str(&pipeline_contents)?;
    
    // read in stages

    // execute stages as jobs on k8s

    // record results
    Ok(())
}