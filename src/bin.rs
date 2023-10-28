#[cfg(feature = "local")]
use std::net::SocketAddr;

#[cfg(feature = "local")]
use anyhow::Result;

/// Local main entrypoint.
///
/// See also `axum` - these will need to be manuall kept doing the same thing it seems 😭.
#[cfg(feature = "local")]
#[tokio::main]
async fn main() -> Result<()> {
    println!("Serving at: http://127.0.0.1:8000");
    let router = cryptorado_rs::get_router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}

/// The main function for running this the shuttle service.
///
/// See also `main` - these will need to be manuall kept doing the same thing it seems 😭.
#[cfg(not(feature = "local"))]
#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    Ok(cryptorado_rs::get_router().into())
}
