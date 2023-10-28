use axum::Router;
use tower_http::services::ServeDir;

/// Get the service router
pub fn get_router() -> Router {
    Router::new().nest_service("/", ServeDir::new("static"))
}

/// The main function for running this the shuttle service.
///
/// See also `main` in `bin.rs` - these will need to be manuall kept doing the same thing it seems 😭.
#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().nest_service("/", ServeDir::new("static"));

    Ok(router.into())
}
