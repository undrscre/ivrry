pub mod buttons; 
pub mod projects;
pub mod about; 
pub mod blog;
pub mod diagnostic;

pub async fn retrieve_context(name: &str) -> Option<minijinja::value::Value> {
    match name {
        _ => None
    }
}