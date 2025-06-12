use std::{collections::HashMap, sync::{mpsc, LazyLock, RwLock}, path::Path};
use log::{debug, info, log};
use minijinja::Environment;
use notify::{Event, RecursiveMode, Watcher, Result};
use warp::Filter;
use tokio::task;

use crate::builder::generate_page;

static PAGES: LazyLock<RwLock<HashMap<String, String>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

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

    let posts = warp::path::end().and_then(|| get_page("blog.html".to_string()));
    let post = warp::path::param().and_then(|f: String| get_page(format!("blog/{}.html", f)));

    // note; this only reads the first param
    // /blog/asdfghjkl
    //   ^ only this gets read

    let catchall = warp::path::param().and_then(|f: String| get_page(format!("{}.html", f)));

    // filesystem watcher, looks for changes and then updates the hashmap accordingly
    let (tx, rx) = mpsc::channel::<Result<Event>>();
    let mut watcher = notify::recommended_watcher(tx).expect("oops no watcher");
    watcher.watch(Path::new("templates"), RecursiveMode::Recursive).expect("oops no files");

    let mut env = env.clone();
    task::spawn(async move {
        for res in rx {
            let event = res.unwrap();

            if !event.kind.is_modify() {
               continue;
            }

            for path in event.paths {
                let filename = path.file_name().unwrap().to_str().unwrap();
                debug!("changed file: {}", filename);
                
                let source = std::fs::read_to_string(&path).unwrap();

                env.remove_template(filename);
                env.add_template_owned(filename.to_owned(), source).unwrap();

                let result = generate_page(&env, filename).await.unwrap();
                PAGES.write().unwrap().insert(filename.to_owned(), result);
                info!("{} reloaded", filename);
            }
        }
    });

    info!("started server at http://127.0.0.1:3030");
    let assets = warp::path("assets").and(warp::fs::dir("assets"));
    let blog = warp::path("blog").and(posts.or(post));
    let routes = assets.or(index).or(blog).or(catchall);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

