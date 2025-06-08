use std::fs;

use minijinja::Environment;
use walkdir::WalkDir;

use crate::contexts::PageContext;

// only used once hopefully,
pub fn get_environment() -> Environment<'static> {
    let mut env = Environment::new();
    for entry in WalkDir::new("templates").min_depth(1).max_depth(2) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            // what the fuck
            let name = entry.path().to_str().unwrap().to_owned();
            let source = fs::read_to_string(&entry.path()).expect("unable to get source");
            env.add_template_owned(name.clone(), source).expect("unable to create template");

            println!("{}", name);
        }
    }
    env
}

pub fn generate_page<P: PageContext>(env: &Environment, page: &str) -> String {
    let tmpl = env.get_template(page).expect("unable to get template, is it real?");
    tmpl.render(P::context()).expect("unable to render template")
}