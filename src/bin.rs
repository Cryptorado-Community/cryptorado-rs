#[cfg(feature = "local")]
use std::net::SocketAddr;

#[cfg(feature = "local")]
use axum;

#[cfg(feature = "local")]
use anyhow::Result;

#[cfg(feature = "local")]
#[tokio::main]
async fn main() -> Result<()> {
    println!("Serving at: http://127.0.0.1:5280");
    let router = cryptorado_rs::get_router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 5280));
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}

/// Dummy main, should never be encountered!
///
/// See also `main` in `lib.rs` - these will need to be manuall kept doing the same thing it seems 😭.
#[cfg(not(feature = "local"))]
fn main() {
    // dummy main function if shuttle is used
    panic!("This main() should never run! See lib.rs for the shuttle main()")
}
