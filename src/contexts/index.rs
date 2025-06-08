use minijinja::context;

pub fn context() -> minijinja::value::Value {
    context! { title => "Hi" }
}