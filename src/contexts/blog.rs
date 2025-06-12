use std::fs;

use minijinja::context;
use pulldown_cmark::Parser;

use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::{Deserialize, Serialize};

use chrono::NaiveDate;
use std::sync::LazyLock;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Meta {
    title: String,
    desc: String,
    date: NaiveDate,
    blog_tags: Vec<String>,
    published: bool,
}

#[derive(Serialize, Clone, Debug)]
pub struct Post {
    pub meta: Meta,
    pub html: String,
    pub slug: String,
}

pub fn process_posts() -> Vec<Post> {
    let mut posts: Vec<Post> = fs::read_dir("content/posts")
        .unwrap()
        .filter_map(|post| { 
            let post = post.unwrap();
            if post.file_type().unwrap().is_dir() {
                return None;
            }
            let file = fs::read_to_string(post.path()).unwrap();
            let slug = post.file_name().to_str().unwrap().to_owned();

            let matter = Matter::<YAML>::new();
            let meta = matter.parse_with_struct::<Meta>(&file).unwrap();

            let parser = Parser::new(&meta.content);
            let mut html = String::new();
            pulldown_cmark::html::push_html(&mut html, parser);

            Some(Post {
                meta: meta.data,
                html,
                slug: slug.trim_end_matches(".md").to_owned(),
            })
        })
        .collect();

    posts.sort_unstable_by(|a, b| b.meta.date.cmp(&a.meta.date)); 
    posts
}

pub fn context() -> minijinja::value::Value {
    let posts = process_posts();
    context! {
        posts
    }
}