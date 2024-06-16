use std::{sync::Arc, time::Duration};

use axum::{
    body::Body,
    error_handling::{HandleError, HandleErrorLayer},
    extract::{Json, Path, Query, State},
    http::{Request, Response, StatusCode},
    routing::{get, post},
    BoxError, Extension, Router,
};
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

#[derive(Debug)]
struct AppState {
    config: String,
}

pub async fn example() {
    let shared_state = Arc::new(AppState {
        config: "config".into(),
    });

    let some_failable_service = tower::service_fn(|_req| async {
        thing_that_might_fail().await?;
        Ok::<_, anyhow::Error>(Response::new(Body::empty()))
    });

    let app = Router::new()
        .route("/", get(root))
        .route("/path/:path1/:path2", get(path))
        .route("/query", get(query))
        .route("/json", post(json))
        .with_state(shared_state)
        .route_service(
            "/failable",
            // register some failable service and its error handler
            HandleError::new(some_failable_service, handle_anyhow_error),
        )
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::new())
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(30))
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}

pub async fn root(State(state): State<Arc<AppState>>) {
    let a = state.clone();
}

async fn path(Path((path1, path2)): Path<(String, String)>) {
    println!("{path1}, {path2}");
}

#[derive(Deserialize, Debug)]
struct Pagination {
    page: usize,
    per_page: usize,
}
async fn query(pagination: Query<Pagination>) {
    println!("{pagination:?}");
}

#[derive(Serialize, Debug)]
struct User {
    id: Uuid,
    email: String,
}

#[derive(Deserialize, Debug)]
struct CreateUser {
    email: String,
    password: String,
}

// deserialize request body
async fn json(Json(payload): Json<CreateUser>) -> Json<User> {
    let user = User {
        id: Uuid::new_v4(),
        email: payload.email.clone(),
    };
    println!("{payload:?}");

    Json(user)
}

async fn thing_that_might_fail() -> Result<(), anyhow::Error> {
    // do something
    Ok(())
}

async fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {err}"),
    )
}

async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<tower::timeout::error::Elapsed>() {
        (StatusCode::REQUEST_TIMEOUT, "Request Timeout".to_string())
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Internal Server Error: {err}"),
        )
    }
}
