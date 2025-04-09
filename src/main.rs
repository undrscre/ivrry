use minijinja::Environment;
use warp::Filter;
use std::fs;
mod routes;

use minijinja::{Value, Error};

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
            println!("added template {}", name);
            env.add_template_owned(name, source).unwrap();
        }
    });
    env
}

#[tokio::main]
async fn main() {
    let pages = routes::pages();
    let assets = warp::path("assets").and(warp::fs::dir("assets"));

    let routes = assets.or(pages);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}