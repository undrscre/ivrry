use askama_warp::Template;
use warp::reply::Reply;

#[derive(Template)]
#[template(path="projects.html")]
pub struct ProjectsPage {}

pub async fn page() -> impl Reply {
    ProjectsPage {}
}
