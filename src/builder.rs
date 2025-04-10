use crate::routes::{about, blog, buttons, index, projects};
use std::fs;
const OUT_DIR: &str = "dist";

macro_rules! generate_page {
    ($module:ident, $filename:expr) => {
        let content = $module::page_html().await;
        fs::write(format!("{}/{}", OUT_DIR, $filename), content).unwrap();
    };
}

pub async fn build() -> Result<(), std::io::Error> {
    fs::create_dir_all(OUT_DIR)?;
    fs::create_dir_all(format!("{}/assets", OUT_DIR))?;
    fs::create_dir_all(format!("{}/blog", OUT_DIR))?;

    let mut options = fs_extra::dir::CopyOptions::new();
    options.overwrite = true;
    fs_extra::dir::copy("assets", OUT_DIR, &options)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    generate_page!(index, "index.html");
    generate_page!(about, "about.html");
    generate_page!(buttons, "buttons.html");
    generate_page!(projects, "projects.html");
    generate_page!(blog, "blog/index.html");

    let posts = fs::read_dir("content/posts")?;
    for post in posts {
        let post = post?;
        let slug = post.file_name().to_str().unwrap().trim_end_matches(".md").to_owned();

        let content = blog::post_html(&slug).await;
        fs::write(format!("{}/blog/{}.html", OUT_DIR, &slug), content)?;
    }

    Ok(())
}