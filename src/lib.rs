use std::{sync::{Arc, RwLock}, collections::HashSet};

use s3::Bucket;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub mod pipeline;
pub mod config;
pub mod client;
pub mod server;
pub mod health;
mod git;
mod kube;
mod s3buckets;
mod utils;

#[derive(Clone)]
pub struct ConstructumState {
    pub postgres: Pool<Postgres>,
    pub s3_bucket: Bucket,
    pub git_server_url: String,
    pub current_jobs: Arc<RwLock<HashSet<Uuid>>>,
    pub container_name: String,
}

impl ConstructumState {
    pub fn new(pool: Pool<Postgres>, s3_bucket: Bucket, git_url: String, container_name: String) -> ConstructumState {
        ConstructumState { postgres: pool, s3_bucket, git_server_url: git_url, current_jobs: Arc::new(RwLock::new(HashSet::new())), container_name: container_name }
    }
}