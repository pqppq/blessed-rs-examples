// mod anyhow;
// mod thiserror;
// mod tracing;
// mod slog;
// mod itertools;
// mod polars;
// mod tokio;
// mod reqwest;
// mod axum;
mod sqlx;

#[tokio::main]
async fn main() {
    // anyhow::example();
    // thiserror::example();
    // tracing::example();
    // slog::example();
    // itertools::example();
    // polars::example();
    // tokio::example();
    // reqwest::example().await;
    // axum::example().await;
    sqlx::example().await;
}
