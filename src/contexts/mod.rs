pub mod index;

pub fn retrieve_context(name: &str) -> Option<minijinja::value::Value> {
    match name {
        "test/index.html" => Some(index::context()),
        _ => None
    }
}