
use axum::{
    routing::{get, post, delete}, Router,
};
use constructum::{config::{Config, ConstructumConfigError}, ConstructumState, server::restart_unfinished_jobs};
use tokio_cron_scheduler::{JobScheduler, Job};
use tower_http::{trace::TraceLayer, classify::StatusInRangeAsFailures};

use std::{net::SocketAddr, time::Duration};

#[tokio::main]
async fn main() -> Result<(), ConstructumConfigError> {
    tracing_subscriber::fmt::init();
    // let sched = JobScheduler::new().await.expect("failed to make scheduler");

    let config = match envy::prefixed("CONSTRUCTUM_").from_env::<Config>() {
        Ok(cfg) => cfg,
        Err(err) => panic!("{err:#?}"),
    };

    let gsu = config.git_server_url.clone().expect("failed to find git server URL");
    let container_name = config.container_name.clone();
    let (pool, bucket) = constructum::config::build_postgres_and_s3(config).await?;

    let state = ConstructumState::new(pool, bucket, gsu, container_name);

    // TODO: invert control for endpoints.
    let app = Router::new()
        .route("/health", get(constructum::health))
        .route("/v1/webhook", post(constructum::webhook::webhook))
        .route("/v1/jobs", get(constructum::server::api::job::endpoints::list_jobs))
        .route("/v1/job/:job_id", get(constructum::server::api::job::endpoints::get_job))
        .route("/v1/job/:job_id/logs", get(constructum::server::api::job::endpoints::get_job_logs))
        .route("/v1/repos/:repo_id", get(constructum::server::api::repo::endpoints::get_repo))
        .route("/v1/repos/:repo_id", delete(constructum::server::api::repo::endpoints::remove_repository))
        .route("/v1/repos", get(constructum::server::api::repo::endpoints::list_all_repos))
        .route("/v1/repos", post(constructum::server::api::repo::endpoints::register_repository))
        .route("/v1/known_repos", get(constructum::server::api::repo::endpoints::list_known_repos))
        .layer(TraceLayer::new(
            StatusInRangeAsFailures::new(400..=599).into_make_classifier()
        ))
        .with_state(state.clone());
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    tracing::debug!("listening on {}", addr);

    // sched.add(Job::new_repeated_async(Duration::from_secs(300), move |_uuid, _l| {
    //     let cloned_state = state.clone();
    //     Box::pin(async move {
    //         println!("restarting an unfinished job");
    //         restart_unfinished_jobs(cloned_state).await.expect("Failed to restart unfinished jobs");
    //     }) 
    // }).expect("failed to build job")).await.expect("failed to schedule job");

    // sched.start().await.expect("failed to start scheduler");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}