use std::time::Duration;

use axum::{Json, Router, http::StatusCode, routing::get};
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};

use iam::presentation::web::router::iam_router;
use utoipa::OpenApi;

use crate::{openapi::ApiDoc, state::AppState};

pub fn create_router(global_state: AppState) -> Router {
    let cors = CorsLayer::permissive();
    let timeout =
        TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(5));

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route(
            "/api-docs/openapi.json",
            get(|| async { Json(ApiDoc::openapi()) }),
        )
        .nest("/api/v1/iam", iam_router(global_state.pool.clone()))
        .layer(timeout)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}
