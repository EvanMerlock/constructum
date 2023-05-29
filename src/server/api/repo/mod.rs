pub mod db;
pub mod endpoints;
mod model;
mod system;

use axum::routing::{get, delete, post};

use crate::ConstructumServerState;

pub use self::model::*;

pub fn register_module(router: axum::Router<ConstructumServerState, axum::body::Body>) -> axum::Router<ConstructumServerState, axum::body::Body> {
    router
        .route("/repos/:repo_id", get(self::endpoints::get_repo))
        .route("/repos/:repo_id", delete(self::endpoints::remove_repository))
        .route("/repos/:repo_id/jobs", get(self::endpoints::jobs_for_repository))
        .route("/repos", get(self::endpoints::list_all_repos))
        .route("/repos", post(self::endpoints::register_repository))
        .route("/known_repos", get(self::endpoints::list_known_repos))
}