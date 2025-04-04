use std::fs;

use askama_warp::Template;
use warp::reply::Reply;
use serde::Deserialize;

#[derive(Deserialize)]
struct Album {
    url: String,
    title: String,
    artwork: String,
    artist: String,
}

#[derive(Template)]
#[template(path="about.html")]
pub struct AboutPage {
    albums: Vec<Album>
}

pub async fn page() -> impl Reply {
    let file = 
        fs::read_to_string("content/data/albums.json")
        .expect("Unable to read albums.json");
    let albums: Vec<Album> = 
        serde_json::from_str(&file)
        .expect("JSON was not formatted correctly");
    
    AboutPage {
        albums
    }
}