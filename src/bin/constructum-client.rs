
use axum::{
    routing::{get}, Router,
};
use constructum::{client::{execute_pipeline, create_client_job, ConstructumClientError}, config::Config};

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), ConstructumClientError> {
    tracing_subscriber::fmt::init();

    let config = match envy::prefixed("CONSTRUCTUM_").from_env::<Config>() {
        Ok(cfg) => cfg,
        Err(err) => panic!("{err:#?}"),
    };

    let app = Router::new()
        .route("/", get(constructum::root));

    let task = tokio::spawn(async {
        create_client_job(config).await
    });

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}