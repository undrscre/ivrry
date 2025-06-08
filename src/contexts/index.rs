use crate::contexts::PageContext;

pub struct Index {}

impl PageContext for Index {
    fn context() -> minijinja::value::Value {
        minijinja::value::Value::from_serialize(serde_json::json!({
            "title": "Welcome to the Index Page",
            "description": "This is an example context output for the index page.",
            "items": ["item1", "item2", "item3"]
        }))
    }
}