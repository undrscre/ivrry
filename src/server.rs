use crate::builder::generate_page;
use log::info;
use minijinja::Environment;
use notify::{Event, RecursiveMode, Result, Watcher};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{LazyLock, RwLock, mpsc},
};
use tokio::task;
use warp::Filter;

static PAGES: LazyLock<RwLock<HashMap<String, String>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

// helper function
async fn get_page(path: String) -> std::result::Result<impl warp::Reply, warp::Rejection> {
    let pages = PAGES.read().expect("unable to read pages");
    info!("reading path {}", path);
    pages
        .get(path.as_str())
        .map(|c| warp::reply::html(c.clone()))
        .ok_or_else(|| warp::reject::not_found())
}

// this only serves in memory
pub async fn serve_pages(inp: HashMap<String, String>, env: &Environment<'static>) {
    *PAGES.write().unwrap() = inp;

    let index = warp::path::end().and_then(|| get_page("index.html".to_string()));
    let catchall = warp::path::full().and_then(|full_path: warp::path::FullPath| {
        let path = full_path.as_str().trim_start_matches('/').to_string();
        async move {
            match get_page(format!("{}.html", path)).await {
                Ok(reply) => Ok(reply),
                Err(_) => get_page("not_found.html".to_string()).await,
            }
        }
    });

    // note; this only reads the first param
    // /blog/asdfghjkl
    //   ^ only this gets read

    // let catchall = warp::path::param().and_then(|f: String| get_page(format!("{}.html", f)));

    // filesystem watcher, looks for changes and then updates the hashmap accordingly
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx).expect("oops no watcher");
    watcher
        .watch(Path::new("templates"), RecursiveMode::Recursive)
        .expect("oops no files");

    let mut env = env.clone();
    task::spawn(async move {
        for res in rx {
            let event = res.unwrap();

            if !event.kind.is_modify() {
                continue;
            }

            for path in event.paths {
                // Get the relative path from "templates" directory
                let rel_path: PathBuf = path
                    .strip_prefix(std::env::current_dir().unwrap().join("templates"))
                    .or_else(|_| path.strip_prefix("templates"))
                    .unwrap_or(&path)
                    .to_path_buf();

                let filename = rel_path.as_os_str().to_str().unwrap();
                let source = std::fs::read_to_string(&path).unwrap();

                env.remove_template(filename);
                env.add_template_owned(filename.to_owned(), source).unwrap();

                let result = generate_page(&env, filename).await.unwrap();
                PAGES.write().unwrap().insert(filename.to_owned(), result);
                info!("{} reloaded", filename);

                info!("{:#?}", rel_path);
            }
        }
    });

    info!("started dev server at http://127.0.0.1:3030");
    let assets = warp::path("assets").and(warp::fs::dir("assets"));
    // let blog = warp::path("blog").and(posts.or(post));
    let routes = assets.or(index).or(catchall);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

pub async fn serve_folder(folder: String) {
    let folder = warp::fs::dir(folder);
    info!("started preview server at http://127.0.0.1:3030");
    warp::serve(folder).run(([127, 0, 0, 1], 3030)).await;
}
