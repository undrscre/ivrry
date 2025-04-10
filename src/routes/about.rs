use warp::reply::Reply;
use serde::{Serialize, Deserialize};
use std::fs;
use crate::get_env;
use minijinja::context;
#[derive(Serialize, Deserialize)]
struct Album {
    url: String,
    title: String,
    artwork: String,
    artist: String,
}

pub async fn page_html() -> String {
    let file = 
        fs::read_to_string("content/data/albums.json")
        .expect("Unable to read albums.json");
    let albums: Vec<Album> = 
        serde_json::from_str(&file)
        .expect("JSON was not formatted correctly");
    
    let env = get_env();
    let tmpl = env.get_template("about.html").expect("failed to get template");
    let html = tmpl.render(context! {
        albums => albums
    }).expect("unable to render");

    html
}

pub async fn page() -> impl Reply {
    let html = page_html().await;
    warp::reply::html(html)
}