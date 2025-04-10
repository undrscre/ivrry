use crate::get_env;
use minijinja::context;
use serde::{Deserialize, Serialize};
use std::fs;
use warp::reply::Reply;

#[derive(Serialize, Deserialize)]
struct Button {
    url: String,
    name: String,
    art: String,
}

#[derive(Serialize, Deserialize)]
struct Buttons {
    friends: Vec<Button>,
    recco: Vec<Button>,
    others: Vec<Button>,
}

pub async fn page_html() -> String {
    let file =
        fs::read_to_string("content/data/buttons.json").expect("Unable to read buttons.json");
    let buttons: Buttons = serde_json::from_str(&file).expect("JSON was not formatted correctly");
    let env = get_env();
    let tmpl = env
        .get_template("buttons.html")
        .expect("failed to get template");
    let html = tmpl
        .render(context! {
            buttons => buttons
        })
        .expect("unable to render");

    html
}

pub async fn page() -> impl Reply {
    let html = page_html().await;
    warp::reply::html(html)
}
