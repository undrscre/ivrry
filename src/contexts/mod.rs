pub mod buttons; 
pub mod projects;
pub mod about; 
pub mod blog;
pub mod diagnostic;

pub async fn retrieve_context(name: &str) -> Option<minijinja::value::Value> {
    match name {
        "buttons.html" => Some(buttons::context()),
        "projects.html" => Some(projects::context().await),
        "about.html" => Some(about::context()),
        "blog.html" => Some(blog::context()),
        "diagnostic.html" => Some(diagnostic::context().await),
        _ => None
    }
}