use std::{sync::{Arc, RwLock}, collections::HashSet, ops::Deref};

use s3::Bucket;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{ConstructumSharedState, config::{Config, ConstructumConfigError}};


#[derive(Clone)]
pub struct ConstructumServerState {
    git_server_url: String,
    current_jobs: Arc<RwLock<HashSet<Uuid>>>,
    shared: ConstructumSharedState,
}

impl ConstructumServerState {
    pub async fn new(config: Config) -> Result<ConstructumServerState, ConstructumConfigError> {
        let css = ConstructumSharedState::from(&config).await?;

        let gsu = config
        .git_server_url
        .clone()
        .expect("failed to find git server URL");

        Ok(ConstructumServerState { shared: css, git_server_url: gsu, current_jobs: Arc::new(RwLock::new(HashSet::new())) })
    }

    pub fn git_server_url(&self) -> String {
        self.git_server_url.clone()
    }

    pub fn current_jobs(&self) -> Arc<RwLock<HashSet<Uuid>>> {
        self.current_jobs.clone()
    }
}

impl Deref for ConstructumServerState {
    type Target = ConstructumSharedState;

    fn deref(&self) -> &Self::Target {
        &self.shared
    }
}