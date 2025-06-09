use minijinja::context;
use serde::{Deserialize, Serialize};
use std::fs;
#[derive(Serialize, Deserialize)]
struct Album {
    url: String,
    title: String,
    artwork: String,
    artist: String,
}

pub fn context() -> minijinja::value::Value {
    let file = fs::read_to_string("content/data/albums.json").expect("Unable to read albums.json");
    let albums: Vec<Album> = serde_json::from_str(&file).expect("JSON was not formatted correctly");

    context! {
        albums => albums
    }
}
