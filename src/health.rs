use axum::routing::get;

use crate::ConstructumState;

pub async fn health() -> &'static str {
    "Ok"
}