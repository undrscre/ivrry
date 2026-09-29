mod buttons;
mod about;

pub async fn retrieve_context(name: &str) -> Option<minijinja::value::Value> {
    match name {
        "about.html" => Some(about::context().await),
        "friends.html" => Some(buttons::context().await),
        _ => None
    }
}