pub mod webhook;
pub mod pipeline;

pub async fn root() -> &'static str {
    "Constructum Root"
}