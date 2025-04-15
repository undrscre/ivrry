pub mod about;
pub mod blog;
pub mod buttons;
pub mod index;
pub mod projects;
pub mod diagnostic;
pub mod not_found;
pub mod guestbook;

use warp::Filter;
use warp::http::StatusCode;

pub fn pages() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let index = warp::path::end().then(index::page);
    let about = warp::path("about").then(about::page);
    let buttons = warp::path("buttons").then(buttons::page);
    let projects = warp::path("projects").then(projects::page);
    let guestbook = warp::path("guestbook").then(guestbook::page);
    let build = warp::path("build").then(diagnostic::page);

    let posts = warp::path::end().then(blog::posts);
    let post = warp::path::param().then(blog::post);
    let blog = warp::path("blog").and(posts.or(post));

    index.or(projects)
        .or(about)
        .or(buttons)
        .or(blog)
        .or(guestbook)
        .or(build)
}


pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    if err.is_not_found() {
        let html = not_found::page_html().await;
        Ok(warp::reply::with_status(warp::reply::html(html), StatusCode::NOT_FOUND))
    } else {
        eprintln!("Unhandled rejection: {:?}", err);
        Err(err)
    }
}