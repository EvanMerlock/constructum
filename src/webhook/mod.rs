use std::{path::Path};

use axum::{Json};
use tokio::{fs::File, io::AsyncReadExt};

use crate::pipeline::Pipeline;

use self::{payload::GitWebhookPayload, error::ConstructumWebhookError};

pub mod payload;
pub mod error;


pub async fn webhook(Json(payload): Json<GitWebhookPayload>) -> axum::response::Result<(), ConstructumWebhookError> {
    let pipeline_file_location = Path::new("E:\\Code\\.constructum\\build_cache");
    tokio::fs::create_dir_all(pipeline_file_location).await?;
    // clone repo
    
    let mut git_init_repo = tokio::process::Command::new("git");
    git_init_repo.arg("init");
    git_init_repo.current_dir(pipeline_file_location);
    git_init_repo.spawn()?.wait().await?;

    let mut git_add_remote_repo = tokio::process::Command::new("git");
    git_add_remote_repo.args(["remote", "add", "origin", &payload.repository.html_url]);
    git_add_remote_repo.current_dir(pipeline_file_location);
    git_add_remote_repo.spawn()?.wait().await?;

    let mut git_fetch_repo = tokio::process::Command::new("git");
    git_fetch_repo.args(["fetch", "--tags", "--depth", "100", "origin"]);
    git_fetch_repo.current_dir(pipeline_file_location);
    git_fetch_repo.spawn()?.wait().await?;

    let mut git_detach_repo = tokio::process::Command::new("git");
    git_detach_repo.args(["reset", "--hard", &payload.after]);
    git_detach_repo.current_dir(pipeline_file_location);
    git_detach_repo.spawn()?.wait().await?;

    // check for .constructum.yml
    let pipeline_file_path = Path::new("E:\\Code\\.constructum\\build_cache\\.constructum.yml");
    let mut pipeline_file = File::open(pipeline_file_path).await?;

    let mut pipeline_contents = String::new();
    pipeline_file.read_to_string(&mut pipeline_contents).await?;

    let pipeline: Pipeline = serde_yaml::from_str(&pipeline_contents)?;
    println!("{pipeline:?}");
    
    // read in stages

    // execute stages as jobs on k8s

    // record results
    Ok(())
}