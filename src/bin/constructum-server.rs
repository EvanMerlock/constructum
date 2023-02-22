
use axum::{
    routing::{get, post}, Router,
};
use constructum::{config::{Config, ConstructumConfigError}, ConstructumState};

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), ConstructumConfigError> {
    tracing_subscriber::fmt::init();

    let config = match envy::prefixed("CONSTRUCTUM_").from_env::<Config>() {
        Ok(cfg) => cfg,
        Err(err) => panic!("{err:#?}"),
    };

    let container_name = config.container_name.clone();
    let (pool, bucket) = constructum::config::build_postgres_and_s3(config).await?;

    let app = Router::new()
        .route("/", get(constructum::root))
        .route("/webhook", post(constructum::webhook::webhook))
        .with_state(ConstructumState::new(pool, bucket, container_name));
    
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
