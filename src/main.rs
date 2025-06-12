mod builder;
mod contexts;
mod server;

use crate::{builder::{build_all, get_environment}, server::serve_pages};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("site=debug")).init();
    let env = get_environment();
    let args = std::env::args();

    let pages = build_all(&env).await?;
    serve_pages(pages, &env).await;
    Ok(())
}
