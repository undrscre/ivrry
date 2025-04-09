use crate::get_env;
use minijinja::context;
use warp::reply::Reply;

pub async fn page() -> impl Reply {
    let env = get_env();
    let tmpl = env.get_template("index").unwrap();
    let html = tmpl.render(context!{}).unwrap();

    warp::reply::html(html)
}