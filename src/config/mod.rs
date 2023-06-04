use std::{fmt::Display, error::Error};

use s3::{Bucket, Region, creds::Credentials};
use serde::Deserialize;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub sql_connection_url: String,
    pub s3_region: String,
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub redis_url: String,
    pub container_name: String,
    pub build_cache_location: Option<String>,
    pub pipeline_uuid: Option<String>,
    // only required if secrets are needed for builds
    pub vault_server: Option<String>,
    // required on the server only
    pub git_server_url: Option<String>,
}

pub async fn build_database_clients(config: &Config) -> Result<(Pool<Postgres>, Bucket, redis::Client), ConstructumConfigError> {
    let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&config.sql_connection_url).await?;

    let bucket = Bucket::new(
        &config.s3_bucket,
        Region::Custom {
            region: config.s3_region.clone(),
            endpoint: config.s3_endpoint.clone(),
        },
        Credentials::default().expect("Failed to get credentials"),
    )?
    .with_path_style();

    let redis_client = redis::Client::open(config.redis_url.clone())?;

    Ok((pool, bucket, redis_client))
}

#[derive(Debug)]
pub enum ConstructumConfigError {
    SqlxError(sqlx::Error),
    S3Error(s3::error::S3Error),
    RedisError(redis::RedisError),
}

impl Display for ConstructumConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConstructumConfigError::SqlxError(sql) => write!(f, "Constructum Config Error: SQL Error: {sql}"),
            ConstructumConfigError::S3Error(s3) => write!(f, "Constructum Config Error: S3 Error: {s3}"),
            ConstructumConfigError::RedisError(red) => write!(f, "Constructum Config Error: Redis Error: {red}"),
        }
    }
}

impl Error for ConstructumConfigError {}

impl From<sqlx::Error> for ConstructumConfigError {
    fn from(value: sqlx::Error) -> Self {
        ConstructumConfigError::SqlxError(value)
    }
}

impl From<s3::error::S3Error> for ConstructumConfigError {
    fn from(value: s3::error::S3Error) -> Self {
        ConstructumConfigError::S3Error(value)
    }
}

impl From<redis::RedisError> for ConstructumConfigError {
    fn from(value: redis::RedisError) -> Self {
        ConstructumConfigError::RedisError(value)
    }
}