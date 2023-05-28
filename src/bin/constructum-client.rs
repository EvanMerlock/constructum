
use axum::{
    routing::{get}, Router,
};
use constructum::{client::{create_client_job, ConstructumClientError}, config::Config};

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), ConstructumClientError> {
    tracing_subscriber::fmt::init();

    let config = match envy::prefixed("CONSTRUCTUM_").from_env::<Config>() {
        Ok(cfg) => cfg,
        Err(err) => panic!("{err:#?}"),
    };

    let app = Router::new()
        .route("/health", get(constructum::health::health));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);

    tokio::select! {
        result = create_client_job(config) => {
            result.expect("failed to execute client job");
        }
        _ = axum::Server::bind(&addr).serve(app.into_make_service()) => {
            println!("web server job done");
        }
    }

    Ok(())
}
