use std::fs;
use crate::get_env;

use minijinja::context;
use warp::reply::Reply;
use pulldown_cmark::Parser;

use serde::{Deserialize, Serialize};
use gray_matter::Matter;
use gray_matter::engine::YAML;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
struct Meta {
    title: String,
    desc: String,
    date: String,
    year: u16,
    blog_tags: Vec<String>,
    published: bool
}

#[derive(Serialize, Clone)]
struct Post {
    meta: Meta,
    html: String,
    slug: String,
}

fn process_posts() -> HashMap<String, Post> {
    fs::read_dir("content/posts").unwrap().map(|post| {
        let post = post.unwrap();
        let file = fs::read_to_string(post.path()).unwrap();
        let slug = post.file_name().to_str().unwrap().to_owned();

        let matter = Matter::<YAML>::new();
        let meta = matter.parse_with_struct::<Meta>(&file).unwrap();

        let parser = Parser::new(&meta.content);
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);

        let post = Post { 
            meta: meta.data,
            html,
            slug: slug.trim_end_matches(".md").to_owned()
        };

        (post.slug.clone(), post)
    }).collect::<HashMap<String, Post>>()
}

use std::sync::LazyLock;
static POSTS: LazyLock<HashMap<String, Post>> = LazyLock::new(|| process_posts());

pub async fn posts_html() -> String {
    let posts = POSTS.values().cloned().collect::<Vec<_>>();
    let env = get_env();
    let tmpl = env.get_template("blog.html").unwrap();
    let html = tmpl.render(context!{
        posts => posts
    }).unwrap();
    
    html
}

pub async fn post_html(slug: String) -> String {
    match POSTS.get(slug.as_str()) {
        Some(post) => {
            let env = get_env();
            let tmpl = env.get_template("_post.html").unwrap();
            let html = tmpl.render(context!{
                meta => post.meta,
                article => post.html,
            }).unwrap();

            html
        },
        None => String::new(),
    }
    
}

pub async fn posts() -> impl Reply {
    let html = posts_html().await;
    warp::reply::html(html)
}

pub async fn post(slug: String) -> impl Reply {
    let html = post_html(slug).await;
    warp::reply::html(html)
}