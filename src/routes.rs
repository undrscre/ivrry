pub mod about;
pub mod blog;
pub mod buttons;
pub mod index;
pub mod projects;
pub mod diagnostic;

use warp::Filter;

pub fn pages() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let index = warp::path::end().then(index::page);
    let about = warp::path("about").then(about::page);
    let buttons = warp::path("buttons").then(buttons::page);
    let projects = warp::path("projects").then(projects::page);
    let build = warp::path("build").then(diagnostic::page);

    let posts = warp::path::end().then(blog::posts);
    let post = warp::path::param().then(blog::post);
    let blog = warp::path("blog").and(posts.or(post));

    index.or(projects).or(about).or(buttons).or(blog).or(build)
}
