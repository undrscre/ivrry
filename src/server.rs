use std::{collections::HashMap, sync::{mpsc, LazyLock, RwLock}, path::Path};
use minijinja::Environment;
use notify::{Event, RecursiveMode, Watcher, Result};
use warp::Filter;
use tokio::task;

use crate::builder::generate_page;

static PAGES: LazyLock<RwLock<HashMap<String, String>>> = LazyLock::new(|| RwLock::new(HashMap::new()));

// this only serves in memory
pub async fn serve_pages(inp: HashMap<String, String>, env: &Environment<'static>) {
    *PAGES.write().unwrap() = inp;

    // oops! repeat; what this does is to route / to index.html
    let index = warp::path::end().and_then(move || {
        let pages = PAGES.read().expect("unable to read pages").clone();
        async move {
            pages.get("index.html")
                .map(|c| warp::reply::html(c.clone()))
                .ok_or_else(|| warp::reject::not_found())
        }
    });
    
    // handles the rest of the routes dynamically; *MIGHT* have a vuln?? but
    // genuinely i don't care LOL it's a dev env

    let routes = warp::path::param().and_then(move |f: String| {
        let pages = PAGES.read().expect("unable to read pages").clone();
        async move {
            pages
                .get(&(f + ".html")) // <- this is the one i'm talking about
                .map(|c| warp::reply::html(c.clone()))
                .ok_or_else(|| warp::reject::not_found())
        }
    });

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
                println!("changed file: {}", filename);
                
                let source = std::fs::read_to_string(&path).unwrap();

                env.remove_template(filename);
                env.add_template_owned(filename.to_owned(), source).unwrap();

                let result = generate_page(&env, filename).await.unwrap();
                PAGES.write().unwrap().insert(filename.to_owned(), result);
                println!("{} reloaded", filename);
            }
        }
    });

    println!("started server at http://127.0.0.1:3030");
    let assets = warp::path("assets").and(warp::fs::dir("assets"));
    warp::serve(assets.or(index).or(routes))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

