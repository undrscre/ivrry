use minijinja::context;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
struct Button {
    url: String,
    name: String,
    art: String,
}

#[derive(Serialize, Deserialize)]
struct Stamp {
    name: String,
    src: String,
}

#[derive(Serialize, Deserialize)]
struct Buttons {
    friends: Vec<Button>,
    recco: Vec<Button>,
    others: Vec<Button>,
}

pub fn context() -> minijinja::value::Value {
    let file =
        fs::read_to_string("content/data/buttons.json").expect("Unable to read buttons.json");
    let buttons: Buttons = serde_json::from_str(&file).expect("JSON was not formatted correctly");

    let file = fs::read_to_string("content/data/stamps.json").expect("Unable to read stamps.json");
    let stamp: Vec<Stamp> = serde_json::from_str(&file).expect("JSON was not formatted correctly");

    context! {
        buttons => buttons,
        stamps => stamp
    }
}
