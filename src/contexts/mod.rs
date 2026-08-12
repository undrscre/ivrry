mod buttons;
mod about;

pub async fn retrieve_context(name: &str) -> Option<minijinja::value::Value> {
    match name {
        "about.html" => Some(about::context()),
        "index.html" => Some(buttons::context()),
        _ => None
    }
}