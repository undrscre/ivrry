use warp::Filter;
pub mod routes;

#[tokio::main]
async fn main() {
    // let page = routes::other::IndexPage {}.render();
    // println!("{}", page.unwrap())

    let index = warp::path::end().then(routes::index::page);
    let about = warp::path("about").then(routes::about::page);
    let buttons = warp::path("buttons").then(routes::buttons::page);
    let projects = warp::path("projects").then(routes::projects::page);

    let pages = index.or(about).or(buttons).or(projects);
    let assets = warp::path("assets").and(warp::fs::dir("assets"));

    let routes = assets.or(pages);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}