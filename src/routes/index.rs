use askama_warp::Template;
use warp::reply::Reply;

#[derive(Template)]
#[template(path="index.html")]
pub struct IndexPage {}

pub async fn page() -> impl Reply {
    IndexPage {}
}

pub fn render() -> String {
    IndexPage {}.render().unwrap()
}