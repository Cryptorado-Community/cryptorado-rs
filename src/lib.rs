use axum::Router;
use tower_http::services::ServeDir;

/// Get the service router
pub fn get_router() -> Router {
    Router::new().nest_service("/", ServeDir::new("static"))
}
