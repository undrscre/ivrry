use crate::get_env;
use std::fs;

use minijinja::context;
use pulldown_cmark::Parser;
use warp::reply::Reply;

use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::{Deserialize, Serialize};

use chrono::NaiveDate;
use std::sync::LazyLock;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Meta {
    title: String,
    desc: String,
    date: NaiveDate,
    blog_tags: Vec<String>,
    published: bool,
}

#[derive(Serialize, Clone, Debug)]
struct Post {
    meta: Meta,
    html: String,
    slug: String,
}

    fn process_posts() -> Vec<Post> {
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
    
static POSTS: LazyLock<Vec<Post>> = LazyLock::new(|| process_posts());

fn find_post<'a>(posts: &'a [Post], slug: &str) -> Option<&'a Post> {
    posts.iter().find(|post| post.slug == slug)
}

pub async fn page_html() -> String {
    let env = get_env();
    let tmpl = env.get_template("blog.html").unwrap();
    let html = tmpl
        .render(context! {
            posts => *POSTS
        })
        .unwrap();

    html
}

pub async fn post_html(slug: &String) -> String {
    let posts = process_posts();
    if let Some(post) = find_post(&posts, slug) {
        let env = get_env();
        let tmpl = env.get_template("_post.html").unwrap();
        let html = tmpl
            .render(context! {
                meta => post.meta,
                article => post.html,
            })
            .unwrap();

        html
    } else {
        String::new()
    }
}

pub async fn posts() -> impl Reply {
    let html = page_html().await;
    warp::reply::html(html)
}

pub async fn post(slug: String) -> impl Reply {
    let html = post_html(&slug).await;
    warp::reply::html(html)
}
