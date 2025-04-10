use crate::get_env;
use minijinja::context;
use warp::reply::Reply;

pub async fn page_html() -> String {
    let env = get_env();
    let tmpl = env.get_template("index.html").unwrap();
    tmpl.render(context!{}).unwrap()
}

pub async fn page() -> impl Reply {
    let html = page_html().await;
    warp::reply::html(html)
}