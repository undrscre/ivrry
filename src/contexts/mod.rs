mod about;
mod base;
mod buttons;

use minijinja::{context, value::Value};

pub async fn retrieve_context(name: &str) -> Option<Value> {
    let page = match name {
        "_base.html" => None,
        "about.html" => Some(about::context().await),
        "friends.html" | "index.html" => Some(buttons::context().await),
        _ => None,
    };

    let base = base::context().await; 

    Some(match page {
        Some(p) => context! { ..p, ..base },
        None => base,
    })
}