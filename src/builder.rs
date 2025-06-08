use std::fs;

use minijinja::{Environment, context};
use crate::contexts::retrieve_context;
use walkdir::WalkDir;

pub fn get_environment() -> Environment<'static> {
    let mut env = Environment::new();
    for entry in WalkDir::new("templates").min_depth(1).max_depth(2) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            // what the fuck
            let name = entry.path().to_str().unwrap().replace("templates/", "");
            let source = fs::read_to_string(&entry.path()).expect("unable to get source");
            env.add_template_owned(name.clone(), source).expect("unable to create template");
        }
    }
    env
}

pub fn generate_page(env: &Environment, page: &str) -> Result<String, minijinja::Error> {
    let tmpl = env.get_template(page).expect("unable to get template, is it real?");
    let ctx = match retrieve_context(page) {
        Some(p) => p,
        None => context! {}
    };
    Ok(tmpl.render(ctx)?) // ?? what the fuck
}

pub fn build_all(env: &Environment) -> Result<(), minijinja::Error> {
    let _ = env.templates().for_each(|t| {
        println!("generating page {}", t.0);
        let content = generate_page(env, t.0).expect("unable to compile file");
        fs::write(format!("{}/{}", "dist", t.0), content).unwrap();
    });
    Ok(())
}