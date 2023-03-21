
use axum::{
    routing::{get, post}, Router,
};
use constructum::{config::{Config, ConstructumConfigError}, ConstructumState, server::restart_unfinished_jobs};
use tokio_cron_scheduler::{JobScheduler, Job};

use std::{net::SocketAddr, time::Duration};

#[tokio::main]
async fn main() -> Result<(), ConstructumConfigError> {
    tracing_subscriber::fmt::init();
    let sched = JobScheduler::new().await.expect("failed to make scheduler");

    let config = match envy::prefixed("CONSTRUCTUM_").from_env::<Config>() {
        Ok(cfg) => cfg,
        Err(err) => panic!("{err:#?}"),
    };

    let container_name = config.container_name.clone();
    let (pool, bucket) = constructum::config::build_postgres_and_s3(config).await?;

    let state = ConstructumState::new(pool, bucket, container_name);

    let app = Router::new()
        .route("/health", get(constructum::health))
        .route("/webhook", post(constructum::webhook::webhook))
        .route("/jobs", get(constructum::server::api::job::endpoints::list_jobs))
        .route("/job/:job_id", get(constructum::server::api::job::endpoints::get_job))
        .with_state(state.clone());
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);

    sched.add(Job::new_repeated_async(Duration::from_secs(300), move |_uuid, _l| {
        let cloned_state = state.clone();
        Box::pin(async move {
            println!("restarting an unfinished job");
            restart_unfinished_jobs(cloned_state).await.expect("Failed to restart unfinished jobs");
        }) 
    }).expect("failed to build job")).await.expect("failed to schedule job");

    sched.start().await.expect("failed to start scheduler");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
