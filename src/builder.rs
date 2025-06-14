use std::{collections::HashMap, fs};
use log::{debug};

use crate::contexts::{blog::process_posts, retrieve_context};
use minijinja::{Environment, Error, context};
use walkdir::WalkDir;

const OUT_DIR: &str = "dist";

pub fn get_environment() -> Environment<'static> {
    let mut env = Environment::new();
    for entry in WalkDir::new("templates").min_depth(1).max_depth(2) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            // what the fuck
            let name = entry.path().to_str().unwrap().replace("templates/", "");
            let source = fs::read_to_string(&entry.path()).expect("unable to get source");
            env.add_template_owned(name.clone(), source)
                .expect("unable to create template");

            debug!("successfully added {}", name)
        }
    }
    env
}

pub async fn generate_page<'a>(env: &'a Environment<'a>, page: &str) -> Result<String, Error> {
    let tmpl = env.get_template(page)?;
    let ctx = match retrieve_context(page).await {
        Some(p) => p,
        None => {
            context! {}
        }
    };

    let result = tmpl.render(ctx)?;
    Ok(result)
}

pub async fn build_all<'a>(env: &'a Environment<'a>) -> Result<HashMap<String, String>, Error> {
    let mut pages: HashMap<String, String> = HashMap::new();

    for template in env.clone().templates() {
        // skip over _ and blog, handle that later
        if template.0.starts_with("_") {
            continue;
        };
        let content = generate_page(&env, template.0).await?;
        pages.insert(template.0.to_owned(), content);
    }

    // blog handling
    for entry in process_posts() {
        // oops oh my god i realized this is essentially just a different hard-written context
        // this tots go against this rewrite's purpose but like whatever :PPP blehhh
        let tmpl = env.get_template("_post.html").expect("unable to retrieve post template");
        let html = tmpl.render(context! {
            meta => entry.meta,
            article => entry.html
        }).expect("unable to render blogpost");
        pages.insert("blog/".to_owned() + entry.slug.as_str() + ".html", html);
    }

    Ok(pages)
}

pub fn consolidate(pages: HashMap<String, String>) -> Result<String, std::io::Error> {
    fs::create_dir_all(OUT_DIR)?;
    fs::create_dir_all(format!("{}/assets", OUT_DIR))?;
    fs::create_dir_all(format!("{}/blog", OUT_DIR))?;

    for page in pages {
        fs::write(format!("{}/{}", OUT_DIR, page.0), page.1)?;
    }

    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    fs_extra::dir::copy("assets", OUT_DIR, &options)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(OUT_DIR.to_string())
}
