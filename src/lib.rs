use s3::Bucket;
use sqlx::{Pool, Postgres};

pub mod webhook;
pub mod pipeline;
pub mod config;
pub mod client;
pub mod server;
mod git;
mod kube;

#[derive(Clone)]
pub struct ConstructumState {
    pub postgres: Pool<Postgres>,
    pub s3_bucket: Bucket,
    pub container_name: String,
}

impl ConstructumState {
    pub fn new(pool: Pool<Postgres>, s3_bucket: Bucket, container_name: String) -> ConstructumState {
        ConstructumState { postgres: pool, s3_bucket, container_name }
    }
}

pub async fn health() -> &'static str {
    "Ok"
}