use askama_warp::Template;
use serde::Deserialize;
use warp::reply::Reply;
use std::fs;

#[derive(Deserialize)]
struct Button {
    url: String,
    name: String,
    art: String
}

#[derive(Deserialize)]
struct Buttons {
    friends: Vec<Button>,
    recco: Vec<Button>,
    others: Vec<Button>,
}

#[derive(Template)]
#[template(path="buttons.html")]
pub struct ButtonsPage {
    buttons: Buttons
}

pub async fn page() -> impl Reply {
    let file = 
        fs::read_to_string("content/data/buttons.json")
        .expect("Unable to read buttons.json");
    let buttons: Buttons  = 
        serde_json::from_str(&file)
        .expect("JSON was not formatted correctly");
    ButtonsPage {
        buttons
    }
}
