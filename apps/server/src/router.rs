use axum::{Router, http::StatusCode, routing::get};
use std::time::Duration;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};

use iam::presentation::web::iam_router;

use crate::state::AppState;

pub fn create_router(global_state: AppState) -> Router {
    let cors = CorsLayer::permissive();
    let timeout =
        TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(5));

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .nest("/api/v1/iam", iam_router(global_state.pool.clone()))
        .layer(timeout)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
