use axum::{
    routing::{delete, get, post},
    Router, ServiceExt,
};
use constructum::{
    config::{Config, ConstructumConfigError},
    server::restart_unfinished_jobs,
    ConstructumServerState,
};

use tracing::{log::LevelFilter, Level};
use tracing_subscriber::prelude::*;

use tokio_cron_scheduler::{Job, JobScheduler};
use tower_http::{
    classify::StatusInRangeAsFailures, normalize_path::NormalizePathLayer, trace::TraceLayer,
};
use tower_layer::Layer;

use std::{net::SocketAddr, time::Duration};

#[tokio::main]
async fn main() -> Result<(), ConstructumConfigError> {
    tracing_subscriber::fmt::init();
    // let sched = JobScheduler::new().await.expect("failed to make scheduler");

    let config = match envy::prefixed("CONSTRUCTUM_").from_env::<Config>() {
        Ok(cfg) => cfg,
        Err(err) => panic!("{err:#?}"),
    };


    let state = ConstructumServerState::new(config).await?;

    // TODO: invert control for endpoints.
    let subrouter = Router::new();
    let subrouter = constructum::server::api::webhook::register_module(subrouter);
    let subrouter = constructum::server::api::job::register_module(subrouter);
    let subrouter = constructum::server::api::repo::register_module(subrouter);

    let app = NormalizePathLayer::trim_trailing_slash().layer(
        Router::new()
            .route("/health", get(constructum::health::health))
            .nest("/api/v1/", subrouter)
            .layer(TraceLayer::new(
                StatusInRangeAsFailures::new(400..=599).into_make_classifier(),
            ))
            .with_state(state.clone()),
    );

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
