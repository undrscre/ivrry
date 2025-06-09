mod builder;
mod contexts;
mod server;

use crate::{builder::{build_all, consolidate, get_environment}, server::serve_pages};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = get_environment();
    let args = std::env::args();

    let pages = build_all(&env).await?;
    serve_pages(pages, &env).await;
    Ok(())
}
