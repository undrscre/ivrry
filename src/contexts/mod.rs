mod index;

pub trait PageContext {
    fn context() -> minijinja::value::Value;
}