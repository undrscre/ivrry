mod builder;
mod contexts;
mod server;
mod publisher;

use log::error;

use crate::{
    builder::{build_all, consolidate, get_environment}, publisher::publish, server::{serve_folder, serve_pages}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("site=debug"))
        .init();
    let env = get_environment();
    let args: Vec<String> = std::env::args().collect();
    match args[1].as_str() {
        "publish" => {
            let pages = build_all(&env).await?;
            let folder = consolidate(pages)?;
            publish(&folder).await?;
        },
        "serve" => {
            let pages = build_all(&env).await?;
            serve_pages(pages, &env).await;
        },
        "preview" => {
            let pages = build_all(&env).await?;
            let folder = consolidate(pages)?;
            serve_folder(folder).await;
        }
        _ => {
            error!("invalid argument, valid arguments; <publish/serve/preview>")
        }
    }
    Ok(())
}
