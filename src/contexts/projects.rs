use minijinja::context;
use reqwest::Client;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Repository {
    name: String,
    description: Option<String>,
    html_url: String,
    language: Option<String>,
    stargazers_count: u32,
    fork: bool,
    image: Option<String>,
}

pub async fn get_repos() -> Result<Vec<Repository>, reqwest::Error> {
    let client = Client::new();
    let response = client
        .get("https://api.github.com/users/undrscre/repos")
        .header(USER_AGENT, "ivrry/1.0")
        .send()
        .await?;

    let text = response.text().await?;
    let repos: Vec<Repository> = serde_json::from_str(&text).expect("oops something wrong happened probably ratelimits lol");
    let result = repos
        .into_iter()
        .map(|mut repo| {
            let og_image_url = format!(
                "https://opengraph.githubassets.com/1/undrscre/{}",
                repo.name
            );
            repo.image = Some(og_image_url);
            Some(repo)
        })
        .filter_map(|repo| repo)
        .collect();

    Ok(result)
}

pub async fn context() -> minijinja::value::Value {
    let repos = get_repos().await.unwrap_or_else(|err| {
        eprintln!("error fetching repos: {:?}", err);
        vec![]
    });
    
    context! {
        repos => repos
    }
}
