use std::{fmt::Display, error::Error, path::{Path, PathBuf}};

use tokio::fs::File;


#[derive(Debug)]
pub enum GitError {
    IOError(std::io::Error),
    NoConstructumYml,
}

impl Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::IOError(io) => write!(f, "Git Error: IO Error: {io}"),
            GitError::NoConstructumYml => write!(f, "Git Error: No .constructum.yml file found"),
        }
    }
}

impl Error for GitError {}

impl From<std::io::Error> for GitError {
    fn from(value: std::io::Error) -> Self {
        GitError::IOError(value)
    }
}

pub async fn get_pipeline_file(root: &Path, repo_location: String, repo_name: String, commit_hash: String) -> Result<tokio::fs::File, GitError> {
    let pipeline_file_location = create_repo_directory(root, repo_name.clone()).await?;
    // clone repo
    fetch_repo(&pipeline_file_location, &repo_location).await?;

    let mut git_detach_repo = tokio::process::Command::new("git");
    git_detach_repo.args(["checkout", &commit_hash, "--", ".constructum.yml"]);
    git_detach_repo.current_dir(&pipeline_file_location);
    git_detach_repo.spawn()?.wait().await?;

    // check for .constructum.yml
    read_pipeline_file(root, repo_name).await
}

pub async fn pull_repository(root: &Path, repo_location: String, repo_name: String, commit_hash: String) -> Result<(tokio::fs::File, PathBuf), GitError> {
    let pipeline_file_location = create_repo_directory(root, repo_name.clone()).await?;
    // clone repo
    fetch_repo(&pipeline_file_location, &repo_location).await?;

    let mut git_detach_repo = tokio::process::Command::new("git");
    git_detach_repo.args(["reset", "--hard", &commit_hash]);
    git_detach_repo.current_dir(&pipeline_file_location);
    git_detach_repo.spawn()?.wait().await?;

    read_pipeline_file(root, repo_name).await.map(|x| (x, pipeline_file_location))
}
 
async fn fetch_repo(pipeline_file_location: &Path, repo_location: &str) -> Result<(), GitError> {
    let mut git_init_repo = tokio::process::Command::new("git");
    git_init_repo.arg("init");
    git_init_repo.current_dir(pipeline_file_location);
    git_init_repo.spawn()?.wait().await?;


    let mut git_add_remote_repo = tokio::process::Command::new("git");
    git_add_remote_repo.args(["remote", "add", "origin", repo_location]);
    git_add_remote_repo.current_dir(pipeline_file_location);
    git_add_remote_repo.spawn()?.wait().await?;


    let mut git_fetch_repo = tokio::process::Command::new("git");
    git_fetch_repo.args(["fetch", "--tags", "--depth", "100", "origin"]);
    git_fetch_repo.current_dir(pipeline_file_location);
    git_fetch_repo.spawn()?.wait().await?;


    Ok(())
}

async fn create_repo_directory(root: &Path, repo_name: String) -> Result<PathBuf, GitError> {
    let mut pipeline_file_location = root.to_path_buf();
    pipeline_file_location.push(repo_name);
    tokio::fs::create_dir_all(&pipeline_file_location).await?;
    Ok(pipeline_file_location)
}

pub async fn read_pipeline_file(root: &Path, repo_name: String) -> Result<tokio::fs::File, GitError> {
    // check for .constructum.yml
    let mut pipeline_file_location = root.to_path_buf();
    pipeline_file_location.push(repo_name);
    let mut pipeline_constructum_path = pipeline_file_location.clone();
    pipeline_constructum_path.push(".constructum.yml");
    File::open(pipeline_constructum_path).await.map_err(GitError::from)
}