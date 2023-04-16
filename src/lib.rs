use std::{sync::{Arc, RwLock}, collections::HashSet};

use s3::Bucket;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub mod webhook;
pub mod pipeline;
pub mod config;
pub mod client;
pub mod server;
mod git;
mod kube;
mod s3buckets;

#[derive(Clone)]
pub struct ConstructumState {
    pub postgres: Pool<Postgres>,
    pub s3_bucket: Bucket,
    pub current_jobs: Arc<RwLock<HashSet<Uuid>>>,
    pub container_name: String,
}

impl ConstructumState {
    pub fn new(pool: Pool<Postgres>, s3_bucket: Bucket, container_name: String) -> ConstructumState {
        ConstructumState { postgres: pool, s3_bucket, current_jobs: Arc::new(RwLock::new(HashSet::new())), container_name: container_name }
    }
}

pub async fn health() -> &'static str {
    "Ok"
}