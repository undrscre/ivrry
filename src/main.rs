use warp::Filter;
pub mod routes;

#[tokio::main]
async fn main() {
    let pages = routes::pages();
    let assets = warp::path("assets").and(warp::fs::dir("assets"));

    let routes = assets.or(pages);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}