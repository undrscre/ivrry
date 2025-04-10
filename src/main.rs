use minijinja::Environment;
use minijinja::{Error, Value};
use std::fs;
use warp::Filter;
use std::env;

mod builder;
mod routes;

fn unwrap_val(value: Value) -> Result<Value, Error> {
    if let Some(obj) = value.as_object() {
        if let Some(inner) = obj.get_value(&value) {
            return Ok(inner.clone());
        }
    }
    Ok(value)
}

pub fn get_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.add_filter("unwrap", unwrap_val);
    let _ = fs::read_dir("templates").unwrap().for_each(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let name = path.file_name().unwrap().to_str().unwrap().to_string();
            let source = fs::read_to_string(&path).unwrap();
            env.add_template_owned(name, source).unwrap();
        }
    });
    env
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("");

    match mode {
        "preview" => {
            let _ = builder::build().await;
            let route = warp::fs::dir(builder::OUT_DIR);
            warp::serve(route).run(([127, 0, 0, 1], 3030)).await
        },
        "publish" => {
            let _ = builder::build().await;
            let _ = builder::publish("dist").await;
        }
        "serve" | _ => {
            let pages = routes::pages();
            let assets = warp::path("assets").and(warp::fs::dir("assets"));

            let routes = assets.or(pages);
            warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
        }
    }
}
