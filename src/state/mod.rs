mod client;
mod server;

use crate::config::Config;
use crate::config::ConstructumConfigError;

pub use self::client::*;
pub use self::server::*;

use s3::Bucket;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct ConstructumSharedState {
    postgres: Pool<Postgres>,
    s3_bucket: Bucket,
    container_name: String,
}

impl ConstructumSharedState {
    pub fn new(pool: Pool<Postgres>, s3_bucket: Bucket, container_name: String) -> ConstructumSharedState {
        ConstructumSharedState { postgres: pool, s3_bucket, container_name }
    }

    pub async fn from(config: &Config) -> Result<ConstructumSharedState, ConstructumConfigError> {
        let (pool, bucket) = crate::config::build_postgres_and_s3(config).await?;

        Ok(ConstructumSharedState {
            postgres: pool,
            s3_bucket: bucket,
            container_name: config.container_name.clone(),
        })
    }
    
    pub fn postgres(&self) -> Pool<Postgres> {
        self.postgres.clone()
    }
    
    pub fn s3_bucket(&self) -> Bucket {
        self.s3_bucket.clone()
    }

    pub fn container_name(&self) -> String {
        self.container_name.clone()
    }
}