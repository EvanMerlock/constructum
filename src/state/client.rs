use std::ops::Deref;

use crate::{ConstructumSharedState, config::{Config, ConstructumConfigError}};


#[derive(Clone)]
pub struct ConstructumClientState {
    shared: ConstructumSharedState,
}

impl ConstructumClientState {
    pub async fn new(config: Config) -> Result<ConstructumClientState, ConstructumConfigError> {
        let css = ConstructumSharedState::from(&config).await?;

        Ok(ConstructumClientState { shared: css })
    }
}

impl Deref for ConstructumClientState {
    type Target = ConstructumSharedState;

    fn deref(&self) -> &Self::Target {
        &self.shared
    }
}