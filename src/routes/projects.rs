use serde::{Deserialize, Serialize};
use warp::reply::Reply;
use reqwest::header::USER_AGENT;
use minijinja::context;

use crate::get_env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    name: String,
    description: Option<String>,
    html_url: String,
    language: Option<String>,
    stargazers_count: u32,
    image: Option<String>
}

pub async fn get_repos() -> Result<Vec<Repository>, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/users/undrscre/repos")
        .header(USER_AGENT, "ivrry/1.0")
        .send()
        .await?;

    let text = response.text().await?;
    // println!("Raw response: {}", text);
    let repos: Vec<Repository> = serde_json::from_str(&text).unwrap();
    let result = repos
        .into_iter()
        .map(|mut repo| {
            let og_image_url = format!("https://opengraph.githubassets.com/1/undrscre/{}", repo.name);
            repo.image = Some(og_image_url);
            repo
        })
        .collect();
    
    Ok(result)
}

pub async fn page_html() -> String {
    let repos = get_repos().await.unwrap_or_else(|err| {
        eprintln!("error fetching repos: {:?}", err);
        vec![]
    });

    let env = get_env();
    let tmpl = env.get_template("projects.html").expect("failed to get template");
    let html = tmpl.render(context! {
        repos => repos
    }).expect("unable to render");

    html
}

pub async fn page() -> impl Reply {
    let html = page_html().await;
    warp::reply::html(html)
}