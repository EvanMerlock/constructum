pub mod db;
pub mod endpoints;
mod model;

use axum::routing::get;

use crate::ConstructumState;

pub use self::model::*;

pub fn register_module(router: axum::Router<ConstructumState, axum::body::Body>) -> axum::Router<ConstructumState, axum::body::Body> {
    router
        .route("/jobs", get(self::endpoints::list_jobs))
        .route("/jobs/:job_id", get(self::endpoints::get_job))
        .route("/jobs/:job_id/logs", get(self::endpoints::get_job_logs))
}