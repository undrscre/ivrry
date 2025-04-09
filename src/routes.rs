pub mod index;
// pub mod about;
pub mod projects;
// pub mod buttons;

use warp::Filter;

pub fn pages() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let index = warp::path::end().then(index::page);
    // let about = warp::path("about").then(about::page);
    // let buttons = warp::path("buttons").then(buttons::page);
    let projects = warp::path("projects").then(projects::page);

    index.or(projects)
}
