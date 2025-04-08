use askama_warp::Template;
use serde::Deserialize;
use warp::reply::Reply;
use reqwest::header::USER_AGENT;

#[derive(Template)]
#[template(path="projects.html")]
pub struct ProjectsPage {
    repos: Vec<Repository>
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    name: String,
    description: Option<String>,
    html_url: String,
    language: Option<String>,
    stargazers_count: u32,
}

impl Repository {
    pub fn description_or_default(&self) -> &str {
        self.description.as_deref().unwrap_or("No description available")
    }

    pub fn language_or_default(&self) -> &str {
        self.language.as_deref().unwrap_or("Unknown")
    }
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

    let result = serde_json::from_str::<Vec<Repository>>(&text).unwrap();
    Ok(result)
}

pub async fn page() -> impl Reply {
    let repos = get_repos().await.unwrap_or_else(|err| {
        eprintln!("Error fetching repositories: {:?}", err);
        vec![] // Return an empty vector if there's an error
    });
    ProjectsPage {
        repos
    }
}
