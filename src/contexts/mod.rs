mod index;

pub async fn retrieve_context(name: &str) -> Option<minijinja::value::Value> {
    match name {
        "index.html" => Some(index::context()),
        _ => None
    }
}