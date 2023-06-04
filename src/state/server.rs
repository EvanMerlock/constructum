use std::{sync::{Arc, RwLock}, collections::HashSet, ops::Deref};

use uuid::Uuid;

use crate::{ConstructumSharedState, config::{Config, ConstructumConfigError}};


#[derive(Clone)]
pub struct ConstructumServerState {
    git_server_url: String,
    build_cache_location: String,
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

        let bcl = config.build_cache_location.clone().expect("failed to find build cache location");

        Ok(ConstructumServerState { shared: css, git_server_url: gsu, build_cache_location: bcl, current_jobs: Arc::new(RwLock::new(HashSet::new())) })
    }

    pub fn git_server_url(&self) -> String {
        self.git_server_url.clone()
    }

    pub fn current_jobs(&self) -> Arc<RwLock<HashSet<Uuid>>> {
        self.current_jobs.clone()
    }

    pub fn build_cache_location(&self) -> String {
        self.build_cache_location.clone()
    }
}

impl Deref for ConstructumServerState {
    type Target = ConstructumSharedState;

    fn deref(&self) -> &Self::Target {
        &self.shared
    }
}